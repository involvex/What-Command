export interface TldrExample {
  command: string;
  description: string;
}

export interface ParsedTldrPage {
  id: string;
  title: string;
  description: string;
  platform: string[];
  category: string;
  examples: TldrExample[];
  sourcePath: string;
}

const PLATFORM_DIRS: Record<string, string[]> = {
  common: ["common"],
  linux: ["linux"],
  osx: ["osx"],
  windows: ["windows"],
  android: ["android"],
  sunos: ["sunos"],
  freebsd: ["freebsd"],
  netbsd: ["netbsd"],
  openbsd: ["openbsd"],
};

export function platformFromDir(dir: string): string[] {
  return PLATFORM_DIRS[dir] ?? [dir];
}

/** Parse a tldr-pages markdown file into structured examples. */
export function parseTldrPage(content: string, relPath: string): ParsedTldrPage | null {
  const normalized = content.replace(/\r\n/g, "\n");
  const lines = normalized.split("\n");

  const titleLine = lines.find((l) => l.startsWith("# "));
  const title = titleLine?.slice(2).trim() ?? relPath;

  const descParts: string[] = [];
  for (const line of lines) {
    if (line.startsWith(">")) {
      const text = line.slice(1).trim();
      if (!text.startsWith("More information:")) {
        descParts.push(text.replace(/<[^>]+>/g, "").trim());
      }
    } else if (line.startsWith("-") || line.includes("`")) {
      break;
    }
  }
  const description = descParts.join(" ").trim() || title;

  const examples: TldrExample[] = [];
  let currentDesc = "";
  for (const line of lines) {
    if (line.startsWith("- ") && !line.includes("`")) {
      currentDesc = line.slice(2).replace(/:$/, "").trim();
      continue;
    }
    const cmdMatch = line.match(/`([^`]+)`/);
    if (cmdMatch) {
      const command = cmdMatch[1]
        .replace(/\{\{[^}]+\}\}/g, "")
        .replace(/\s+/g, " ")
        .trim();
      if (command) {
        examples.push({
          command,
          description: currentDesc || description,
        });
      }
      currentDesc = "";
    }
  }

  if (!examples.length) {
    return null;
  }

  const parts = relPath.replace(/\\/g, "/").split("/");
  const platformDir = parts.length >= 2 ? parts[parts.length - 2] : "common";
  const fileStem = parts[parts.length - 1]?.replace(/\.md$/, "") ?? "cmd";
  const id = `${platformDir}-${fileStem}`;

  return {
    id,
    title,
    description,
    platform: platformFromDir(platformDir),
    category: fileStem.split("-")[0] ?? "general",
    examples,
    sourcePath: relPath,
  };
}

const DANGEROUS_PATTERNS: RegExp[] = [
  /rm\s+-rf\s+\//,
  /rm\s+-rf\s+\*/,
  /mkfs\./,
  /dd\s+if=/,
  /:\(\)\{\s*:\|:&\s*\};:/,
  /chmod\s+-R\s+777\s+\//,
  /format\s+c:/i,
  /diskpart/i,
];

export function dangerLevel(command: string): number {
  const lower = command.toLowerCase();
  for (const re of DANGEROUS_PATTERNS) {
    if (re.test(lower)) {
      return 3;
    }
  }
  if (/\brm\s+-rf\b/.test(lower) || /\bdd\s+if=/.test(lower)) {
    return 2;
  }
  if (/\brm\s+-/.test(lower) || /\bkill\s+-9\b/.test(lower)) {
    return 1;
  }
  return 0;
}

const FRAMEWORK_RULES: Array<{
  prefix: RegExp;
  id: string;
  name: string;
  icon: string;
}> = [
  { prefix: /^git\b/, id: "git", name: "Git", icon: "git-branch" },
  { prefix: /^docker\b/, id: "docker", name: "Docker", icon: "container" },
  { prefix: /^kubectl\b/, id: "kubernetes", name: "Kubernetes", icon: "layers" },
  { prefix: /^(npm|npx)\b/, id: "npm", name: "npm", icon: "package" },
  { prefix: /^cargo\b/, id: "rust", name: "Rust", icon: "code" },
  { prefix: /^(python|pip)\b/, id: "python", name: "Python", icon: "terminal" },
  { prefix: /^brew\b/, id: "homebrew", name: "Homebrew", icon: "package" },
  { prefix: /^apt\b/, id: "apt", name: "APT", icon: "package" },
  { prefix: /^yarn\b/, id: "yarn", name: "Yarn", icon: "package" },
  { prefix: /^pnpm\b/, id: "pnpm", name: "pnpm", icon: "package" },
  { prefix: /^helm\b/, id: "helm", name: "Helm", icon: "layers" },
  { prefix: /^terraform\b/, id: "terraform", name: "Terraform", icon: "layers" },
  { prefix: /^aws\b/, id: "aws", name: "AWS CLI", icon: "cloud" },
  { prefix: /^gcloud\b/, id: "gcloud", name: "Google Cloud", icon: "cloud" },
];

export function inferFramework(command: string, category: string) {
  for (const rule of FRAMEWORK_RULES) {
    if (rule.prefix.test(command.trim())) {
      return {
        id: rule.id,
        name: rule.name,
        description: `${rule.name} commands`,
        icon: rule.icon,
      };
    }
  }
  return {
    id: category || "shell",
    name: category.charAt(0).toUpperCase() + category.slice(1),
    description: "Shell commands",
    icon: "terminal",
  };
}
