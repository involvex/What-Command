import type {
  Command,
  CommandSuggestion,
  Framework,
  PlaygroundSession,
  SimulateResult,
} from "../types";

export async function searchCommands(query: string, limit = 50): Promise<Command[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command[]>("search_commands", { query, limit });
}

export async function getCommand(id: string): Promise<Command | null> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command | null>("get_command", { id });
}

export async function listCategories(): Promise<string[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string[]>("list_categories");
}

export async function listFrameworks(): Promise<Framework[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Framework[]>("list_frameworks");
}

export async function commandsByFramework(
  frameworkId: string,
  limit = 50,
): Promise<Command[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command[]>("commands_by_framework", { frameworkId, limit });
}

export async function relatedCommands(
  commandId: string,
  limit = 10,
): Promise<Command[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command[]>("related_commands", { commandId, limit });
}

export async function validateCommand(command: string): Promise<[number, string]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<[number, string]>("validate_command", { command });
}

export async function simulateCommand(
  command: string,
  vars: Record<string, string> = {},
): Promise<SimulateResult> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<SimulateResult>("simulate_command_ipc", { command, vars });
}

export async function askAi(
  prompt: string,
  frameworkId?: string,
): Promise<CommandSuggestion> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<CommandSuggestion>("ask_ai", { prompt, frameworkId });
}

export async function explainCommand(command: string): Promise<string> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("explain_command", { command });
}

export async function getSettings() {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<import("../types").AppSettings>("get_settings");
}

export async function saveSettings(
  settings: import("../types").AppSettings,
): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke("save_settings", { settings });
}

export async function savePlaygroundSession(session: PlaygroundSession): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke("save_playground_session", { session });
}

export async function listPlaygroundSessions(limit = 20): Promise<PlaygroundSession[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<PlaygroundSession[]>("list_playground_sessions", { limit });
}

export async function copyToClipboard(text: string): Promise<void> {
  const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
  await writeText(text);
}
