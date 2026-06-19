use std::path::{Path, PathBuf};

use wc_core::config::config_dir;
use wc_core::error::{Result, WcError};

/// Resolve a GGUF file from explicit path, config models dir, or direct path.
pub fn resolve_model_path(model_id: &str, explicit: Option<&str>) -> Option<PathBuf> {
    if let Some(p) = explicit.filter(|s| !s.is_empty()) {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }

    if let Ok(dir) = config_dir() {
        let candidate = dir.join("models").join(format!("{model_id}.gguf"));
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    let direct = PathBuf::from(model_id);
    if direct.is_file() {
        return Some(direct);
    }

    None
}

#[cfg(feature = "local-llm")]
mod gguf {
    use super::*;
    use std::collections::HashMap;
    use std::num::NonZeroU32;
    use std::sync::{Arc, Mutex, OnceLock};
    use llama_cpp_4::context::params::LlamaContextParams;
    use llama_cpp_4::llama_backend::LlamaBackend;
    use llama_cpp_4::llama_batch::LlamaBatch;
    use llama_cpp_4::model::params::LlamaModelParams;
    use llama_cpp_4::model::{AddBos, LlamaModel, Special};
    use llama_cpp_4::sampling::LlamaSampler;

    struct CachedModel {
        backend: LlamaBackend,
        model: LlamaModel,
    }

    static MODELS: OnceLock<Mutex<HashMap<PathBuf, Arc<CachedModel>>>> = OnceLock::new();

    fn models() -> &'static Mutex<HashMap<PathBuf, Arc<CachedModel>>> {
        MODELS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn load_model(path: &Path) -> Result<Arc<CachedModel>> {
        let mut guard = models()
            .lock()
            .map_err(|_| WcError::Ai("local model cache lock poisoned".into()))?;
        if let Some(existing) = guard.get(path) {
            return Ok(Arc::clone(existing));
        }

        let backend = LlamaBackend::init().map_err(|e| WcError::Ai(e.to_string()))?;
        let model = LlamaModel::load_from_file(&backend, path, &LlamaModelParams::default())
            .map_err(|e| WcError::Ai(format!("failed to load GGUF {}: {e}", path.display())))?;
        let cached = Arc::new(CachedModel { backend, model });
        guard.insert(path.to_path_buf(), Arc::clone(&cached));
        Ok(cached)
    }

    pub fn complete(model_path: &Path, prompt: &str, max_tokens: u32) -> Result<String> {
        let cached = load_model(model_path)?;
        let ctx_params = LlamaContextParams::default().with_n_ctx(NonZeroU32::new(2048));
        let mut ctx = cached
            .model
            .new_context(&cached.backend, ctx_params)
            .map_err(|e| WcError::Ai(e.to_string()))?;

        let tokens = cached
            .model
            .str_to_token(prompt, AddBos::Always)
            .map_err(|e| WcError::Ai(e.to_string()))?;
        let n_prompt = tokens.len();

        let mut batch = LlamaBatch::new(2048, 1);
        for (i, &tok) in tokens.iter().enumerate() {
            batch
                .add(tok, i as i32, &[0], i == n_prompt - 1)
                .map_err(|e| WcError::Ai(e.to_string()))?;
        }
        ctx.decode(&mut batch)
            .map_err(|e| WcError::Ai(e.to_string()))?;

        let sampler = LlamaSampler::chain_simple([
            LlamaSampler::temp(0.7),
            LlamaSampler::top_k(40),
            LlamaSampler::dist(0),
        ]);

        let mut pos = n_prompt as i32;
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut out = String::new();

        for _ in 0..max_tokens {
            let token = sampler.sample(&ctx, 0);
            if cached.model.is_eog_token(token) {
                break;
            }

            let bytes = cached
                .model
                .token_to_bytes(token, Special::Plaintext)
                .map_err(|e| WcError::Ai(e.to_string()))?;
            let mut piece = String::new();
            decoder.decode_to_string(&bytes, &mut piece, false);
            out.push_str(&piece);

            batch.clear();
            batch
                .add(token, pos, &[0], true)
                .map_err(|e| WcError::Ai(e.to_string()))?;
            ctx.decode(&mut batch)
                .map_err(|e| WcError::Ai(e.to_string()))?;
            pos += 1;
        }

        Ok(out.trim().to_string())
    }
}

#[cfg(feature = "local-llm")]
pub fn complete(model_path: &Path, prompt: &str, max_tokens: u32) -> Result<String> {
    gguf::complete(model_path, prompt, max_tokens)
}

#[cfg(not(feature = "local-llm"))]
pub fn complete(_model_path: &Path, _prompt: &str, _max_tokens: u32) -> Result<String> {
    Err(WcError::Ai(
        "local GGUF inference requires building wc-ai with the `local-llm` feature".into(),
    ))
}
