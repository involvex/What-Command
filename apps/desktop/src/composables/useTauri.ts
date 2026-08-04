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

export function modelIdFromGgufPath(path: string): string {
  const base = path.split(/[/\\]/).pop() ?? path;
  return base.replace(/\.gguf$/i, "");
}

export async function importGgufModel(sourcePath: string): Promise<string> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("import_gguf_model", { sourcePath });
}

export async function pickGgufModelFile(): Promise<string | null> {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const selected = await open({
    multiple: false,
    filters: [{ name: "GGUF model", extensions: ["gguf"] }],
  });
  if (!selected || Array.isArray(selected)) {
    return null;
  }
  return importGgufModel(selected);
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

export async function recordUsage(commandId: string, action: string): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("record_usage", { commandId, action });
}

export async function getRecentCommands(limit = 10): Promise<Command[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command[]>("get_recent_commands", { limit });
}

export async function getTopCommands(limit = 10, days = 7): Promise<Command[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Command[]>("get_top_commands", { limit, days });
}

export async function listenShowCommandPalette(
  callback: () => void,
): Promise<() => void> {
  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen("show-command-palette", () => callback());
  return unlisten;
}
