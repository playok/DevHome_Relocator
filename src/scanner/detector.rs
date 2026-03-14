use std::path::PathBuf;

use crate::core::{RelocationMethod, RelocationTarget, TargetStatus};

struct ToolDefinition {
    name: &'static str,
    dir_name: &'static str,
    scan_root: ScanRoot,
    method: RelocationMethod,
}

enum ScanRoot {
    UserProfile,
    AppData,
    LocalAppData,
}

const TOOL_DEFINITIONS: &[ToolDefinition] = &[
    ToolDefinition {
        name: "Cargo",
        dir_name: ".cargo",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(), // filled at runtime
        },
    },
    ToolDefinition {
        name: "Rustup",
        dir_name: ".rustup",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "Gradle",
        dir_name: ".gradle",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "JetBrains JDK",
        dir_name: ".jdks",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Go Path",
        dir_name: "go",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "Go Build Cache",
        dir_name: "go-build",
        scan_root: ScanRoot::LocalAppData,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "npm Cache",
        dir_name: "npm-cache",
        scan_root: ScanRoot::AppData,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "pip Cache",
        dir_name: "pip",
        scan_root: ScanRoot::LocalAppData,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "NuGet Packages",
        dir_name: ".nuget",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "Codeium / Windsurf",
        dir_name: ".codeium",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Cursor",
        dir_name: ".cursor",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Windsurf",
        dir_name: ".windsurf",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Claude Code",
        dir_name: ".claude",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Antigravity",
        dir_name: ".antigravity",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Deno",
        dir_name: ".deno",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "pnpm Store",
        dir_name: "pnpm-store",
        scan_root: ScanRoot::LocalAppData,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
    ToolDefinition {
        name: "Maven Local Repo",
        dir_name: ".m2",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "VS Code",
        dir_name: ".vscode",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Theia IDE",
        dir_name: ".theia-ide",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::Junction,
    },
    ToolDefinition {
        name: "Bun",
        dir_name: ".bun",
        scan_root: ScanRoot::UserProfile,
        method: RelocationMethod::EnvVar {
            var_name: String::new(),
        },
    },
];

fn env_var_for_tool(name: &str) -> Option<String> {
    match name {
        "Cargo" => Some("CARGO_HOME".to_string()),
        "Rustup" => Some("RUSTUP_HOME".to_string()),
        "Gradle" => Some("GRADLE_USER_HOME".to_string()),
        "Go Path" => Some("GOPATH".to_string()),
        "Go Build Cache" => Some("GOCACHE".to_string()),
        "npm Cache" => Some("NPM_CONFIG_CACHE".to_string()),
        "pip Cache" => Some("PIP_CACHE_DIR".to_string()),
        "NuGet Packages" => Some("NUGET_PACKAGES".to_string()),
        "Deno" => Some("DENO_DIR".to_string()),
        "pnpm Store" => Some("PNPM_STORE_DIR".to_string()),
        "Bun" => Some("BUN_INSTALL".to_string()),
        _ => None,
    }
}

fn resolve_scan_root(root: &ScanRoot) -> Option<PathBuf> {
    match root {
        ScanRoot::UserProfile => dirs::home_dir(),
        ScanRoot::AppData => std::env::var("APPDATA").ok().map(PathBuf::from),
        ScanRoot::LocalAppData => std::env::var("LOCALAPPDATA").ok().map(PathBuf::from),
    }
}

/// Check if a tool has already been relocated via environment variable.
/// Returns Some(relocated_path) if the env var is set and points to a different location.
fn detect_env_relocation(var_name: &str, default_path: &PathBuf) -> Option<PathBuf> {
    let value = crate::config::get_user_env_var(var_name).ok()?;
    let env_path = PathBuf::from(&value);
    // Consider it relocated if the env var points somewhere other than the default
    if env_path != *default_path && env_path.exists() {
        Some(env_path)
    } else {
        None
    }
}

pub fn scan_targets() -> Vec<RelocationTarget> {
    let mut targets = Vec::new();

    for def in TOOL_DEFINITIONS {
        let Some(root) = resolve_scan_root(&def.scan_root) else {
            continue;
        };

        let default_path = root.join(def.dir_name);

        let method = match &def.method {
            RelocationMethod::EnvVar { .. } => {
                if let Some(var_name) = env_var_for_tool(def.name) {
                    RelocationMethod::EnvVar { var_name }
                } else {
                    RelocationMethod::Junction
                }
            }
            RelocationMethod::Junction => RelocationMethod::Junction,
        };

        // Read current env var value for display
        let env_current_value = match &method {
            RelocationMethod::EnvVar { var_name } => {
                crate::config::get_user_env_var(var_name).ok()
            }
            RelocationMethod::Junction => None,
        };

        // Check if already relocated
        match &method {
            RelocationMethod::EnvVar { var_name } => {
                if let Some(relocated_path) = detect_env_relocation(var_name, &default_path) {
                    targets.push(RelocationTarget {
                        name: def.name.to_string(),
                        current_path: default_path,
                        size_bytes: None,
                        target_path: Some(relocated_path),
                        method,
                        status: TargetStatus::AlreadyMoved,
                        enabled: false,
                        env_current_value,
                        progress: 0.0,
                    });
                    continue;
                }
            }
            RelocationMethod::Junction => {
                if crate::mover::junction::is_junction(&default_path) {
                    if let Ok(real_target) = std::fs::read_link(&default_path) {
                        targets.push(RelocationTarget {
                            name: def.name.to_string(),
                            current_path: default_path,
                            size_bytes: None,
                            target_path: Some(real_target),
                            method,
                            status: TargetStatus::AlreadyMoved,
                            enabled: false,
                            env_current_value,
                            progress: 0.0,
                        });
                        continue;
                    }
                }
            }
        }

        // Not relocated — normal detection
        if !default_path.exists() {
            continue;
        }

        targets.push(RelocationTarget {
            name: def.name.to_string(),
            current_path: default_path,
            size_bytes: None,
            target_path: None,
            method,
            status: TargetStatus::Detected,
            enabled: false,
            env_current_value,
            progress: 0.0,
        });
    }

    // Sort: EnvVar targets first, then Junction targets
    targets.sort_by_key(|t| match &t.method {
        RelocationMethod::EnvVar { .. } => 0,
        RelocationMethod::Junction => 1,
    });

    targets
}
