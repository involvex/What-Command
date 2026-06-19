export interface Command {
  id: string;
  command: string;
  description: string;
  category: string;
  platform: string[];
  danger_level: number;
  source: string;
  updated_at: string;
}

export interface Framework {
  id: string;
  name: string;
  description: string;
  icon: string;
}

export interface SimulateResult {
  explanation: string;
  sample_output: string;
  exit_code: number;
  blocked: boolean;
}

export interface PlaygroundSession {
  id: string;
  command: string;
  transcript: string;
  updated_at: string;
}

export interface AppSettings {
  ai_provider: string;
  ai_model: string;
  fallback_provider?: string | null;
  fallback_model?: string | null;
  opencode_api_key?: string | null;
  kilo_api_key?: string | null;
  local_model_id?: string | null;
  local_model_path?: string | null;
  local_max_tokens?: number | null;
  openai_compat_base_url?: string | null;
  openai_compat_api_key?: string | null;
}

export interface CommandSuggestion {
  command: string;
  explanation: string;
  danger_level: number;
  platforms: string[];
}
