use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelocationMethod {
    EnvVar { var_name: String },
    Junction,
}

impl std::fmt::Display for RelocationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelocationMethod::EnvVar { var_name } => write!(f, "EnvVar ({})", var_name),
            RelocationMethod::Junction => write!(f, "Junction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetStatus {
    Detected,
    AlreadyMoved,
    Configured,
    Moving,
    Moved,
    Failed(String),
    Rolledback,
}

impl std::fmt::Display for TargetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetStatus::Detected => write!(f, "Detected"),
            TargetStatus::AlreadyMoved => write!(f, "Already Moved"),
            TargetStatus::Configured => write!(f, "Configured"),
            TargetStatus::Moving => write!(f, "Moving..."),
            TargetStatus::Moved => write!(f, "Moved"),
            TargetStatus::Failed(reason) => write!(f, "Failed: {}", reason),
            TargetStatus::Rolledback => write!(f, "Rolled back"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocationTarget {
    pub name: String,
    pub current_path: PathBuf,
    pub size_bytes: Option<u64>,
    pub target_path: Option<PathBuf>,
    pub method: RelocationMethod,
    pub status: TargetStatus,
    pub enabled: bool,
    /// Current value of the associated environment variable (if any).
    pub env_current_value: Option<String>,
}

impl RelocationTarget {
    pub fn size_display(&self, scanning_text: &str) -> String {
        match self.size_bytes {
            Some(bytes) => format_bytes(bytes),
            None => scanning_text.to_string(),
        }
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
