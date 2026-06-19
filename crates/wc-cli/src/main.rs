use std::path::PathBuf;
use clap::{Parser, Subcommand};
use wc_core::config::{ensure_db_from_seed, load_config};
use wc_core::db::CommandStore;
use wc_ai::build_router;
use wc_core::models::AiContext;

#[derive(Parser)]
#[command(name = "wc", about = "What Command — CLI helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search the command database
    Search {
        query: String,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Ask AI to suggest a command
    Ask {
        prompt: String,
    },
    /// Explain a command via AI
    Explain {
        command: String,
    },
    /// Refresh bundled command database from seed
    Update,
}

fn db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("what-command")
        .join("commands.db")
}

fn seed_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/commands.db")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    ensure_db_from_seed(&seed_path(), &path)?;
    let store = CommandStore::open(&path)?;
    store.init_schema()?;

    match cli.command {
        Commands::Search { query, limit } => {
            let hits = store.search(&query, limit)?;
            for cmd in hits {
                println!("{} — {}", cmd.command, cmd.description);
            }
        }
        Commands::Ask { prompt } => {
            let config = load_config()?;
            let router = build_router(&config.settings);
            let ctx = AiContext::default();
            match router.generate_command(&prompt, &ctx).await {
                Ok(s) => {
                    println!("{}\n# {}", s.command, s.explanation);
                }
                Err(e) => {
                    eprintln!("AI error: {e}");
                    let stub = wc_ai::router::stub_suggestion(&prompt);
                    println!("{}\n# {}", stub.command, stub.explanation);
                }
            }
        }
        Commands::Explain { command } => {
            let config = load_config()?;
            let router = build_router(&config.settings);
            let ctx = AiContext::default();
            match router.explain_command(&command, &ctx).await {
                Ok(text) => println!("{text}"),
                Err(e) => {
                    eprintln!("AI error: {e}");
                    println!("{}", wc_ai::router::stub_explain(&command));
                }
            }
        }
        Commands::Update => {
            let seed = seed_path();
            if seed.exists() {
                std::fs::copy(&seed, &path)?;
                println!("Updated database from {}", seed.display());
            } else {
                println!("Seed database not found at {}", seed.display());
            }
        }
    }
    Ok(())
}
