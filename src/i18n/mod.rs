#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    Ko,
    En,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::Ko => "한국어",
            Locale::En => "English",
        }
    }
}

pub fn detect_system_locale() -> Locale {
    let lang = std::env::var("LANG")
        .or_else(|_| std::env::var("LANGUAGE"))
        .unwrap_or_default()
        .to_lowercase();
    if lang.starts_with("ko") {
        return Locale::Ko;
    }

    // Windows: check user locale via GetUserDefaultUILanguage
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("Control Panel\\International", KEY_READ)
        {
            if let Ok(locale_name) = key.get_value::<String, _>("LocaleName") {
                if locale_name.to_lowercase().starts_with("ko") {
                    return Locale::Ko;
                }
            }
        }
    }

    Locale::En
}

pub struct Texts {
    pub locale: Locale,
}

impl Texts {
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    // -- App title --
    pub fn app_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "DevHome Relocator - 개발 캐시 이동 도구",
            Locale::En => "DevHome Relocator",
        }
    }

    // -- Header --
    pub fn home_directory(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "홈 디렉토리",
            Locale::En => "Home Directory",
        }
    }

    pub fn free_space_fmt(&self) -> &'static str {
        // used as: format!("{} {}: {} / {}", mount, this, free, total)
        match self.locale {
            Locale::Ko => "여유",
            Locale::En => "Free",
        }
    }

    // -- Target base selector --
    pub fn target_base_directory(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동할 위치:",
            Locale::En => "Target directory:",
        }
    }

    pub fn browse(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "찾아보기...",
            Locale::En => "Browse...",
        }
    }

    pub fn dry_run(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "시뮬레이션 모드 (실제 변경 없음)",
            Locale::En => "Dry Run (no actual changes)",
        }
    }

    // -- Table headers --
    pub fn col_select(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "선택",
            Locale::En => "Select",
        }
    }

    pub fn col_tool(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "도구",
            Locale::En => "Tool",
        }
    }

    pub fn col_current_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "현재 경로",
            Locale::En => "Current Path",
        }
    }

    pub fn col_size(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "크기",
            Locale::En => "Size",
        }
    }

    pub fn col_target_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 경로",
            Locale::En => "Target Path",
        }
    }

    pub fn col_method(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "방식",
            Locale::En => "Method",
        }
    }

    pub fn col_env_var(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "환경변수",
            Locale::En => "Env Variable",
        }
    }

    pub fn col_status(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "상태",
            Locale::En => "Status",
        }
    }

    pub fn env_not_set(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "(미설정)",
            Locale::En => "(not set)",
        }
    }

    // -- Status labels --
    pub fn status_detected(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "발견됨",
            Locale::En => "Detected",
        }
    }

    pub fn status_already_moved(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동됨 (설정 완료)",
            Locale::En => "Already Moved",
        }
    }

    pub fn status_configured(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "설정 완료",
            Locale::En => "Configured",
        }
    }

    pub fn status_moving(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 중...",
            Locale::En => "Moving...",
        }
    }

    pub fn status_moved(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 완료",
            Locale::En => "Moved",
        }
    }

    pub fn status_failed(&self, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("실패: {}", reason),
            Locale::En => format!("Failed: {}", reason),
        }
    }

    pub fn status_rolledback(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "복원됨",
            Locale::En => "Rolled back",
        }
    }

    // -- Scanning --
    pub fn scanning(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "분석 중...",
            Locale::En => "Scanning...",
        }
    }

    // -- Buttons --
    pub fn btn_rescan(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "다시 검색",
            Locale::En => "Rescan",
        }
    }

    pub fn btn_set_target(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "경로 지정",
            Locale::En => "Set Target",
        }
    }

    pub fn btn_start_move(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 시작",
            Locale::En => "Start Move",
        }
    }

    pub fn btn_rollback(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "되돌리기",
            Locale::En => "Undo / Rollback",
        }
    }

    // -- Log panel --
    pub fn log_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "실행 로그",
            Locale::En => "Activity Log",
        }
    }

    // -- Log messages --
    pub fn log_rescanned(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "디렉토리를 다시 검색했습니다.",
            Locale::En => "Directories rescanned.",
        }
    }

    pub fn log_targets_configured(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 경로가 설정되었습니다.",
            Locale::En => "Target paths configured.",
        }
    }

    pub fn log_no_selection(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동할 항목을 선택해주세요.",
            Locale::En => "No targets selected for migration.",
        }
    }

    pub fn log_process_warning(&self, processes: &[String]) -> String {
        match self.locale {
            Locale::Ko => format!(
                "경고: 관련 프로세스가 실행 중입니다: {}",
                processes.join(", ")
            ),
            Locale::En => format!(
                "Warning: conflicting processes detected: {}",
                processes.join(", ")
            ),
        }
    }

    pub fn log_dry_run_move(&self, name: &str, size: &str, target: &str) -> String {
        match self.locale {
            Locale::Ko => format!("[시뮬레이션] {} ({}) -> {}", name, size, target),
            Locale::En => format!("[DRY RUN] Would move {} ({}) -> {}", name, size, target),
        }
    }

    pub fn log_dry_run_env(&self, var_name: &str, value: &str) -> String {
        match self.locale {
            Locale::Ko => format!("[시뮬레이션] {} = {} 설정 예정", var_name, value),
            Locale::En => format!("[DRY RUN] Would set {} = {}", var_name, value),
        }
    }

    pub fn log_migration_complete(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 이동 완료.", name),
            Locale::En => format!("{}: Migration complete.", name),
        }
    }

    pub fn log_migration_failed(&self, name: &str, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 실패 - {}", name, reason),
            Locale::En => format!("{}: Failed - {}", name, reason),
        }
    }

    pub fn log_rolled_back(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 복원되었습니다.", name),
            Locale::En => format!("{}: Rolled back.", name),
        }
    }

    pub fn log_rollback_failed(&self, name: &str, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 복원 실패 - {}", name, reason),
            Locale::En => format!("{}: Rollback failed - {}", name, reason),
        }
    }

    // -- Process warning --
    pub fn process_warning(&self, processes: &str) -> String {
        match self.locale {
            Locale::Ko => format!("경고: 실행 중인 프로세스가 있습니다: {}", processes),
            Locale::En => format!("Warning: active processes detected: {}", processes),
        }
    }

    // -- Process conflict dialog --
    pub fn conflict_dialog_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "파일 잠금 충돌",
            Locale::En => "File Lock Conflict",
        }
    }

    pub fn conflict_dialog_desc(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "다음 프로세스가 파일을 사용 중입니다. 종료 후 재시도하거나 건너뛸 수 있습니다.",
            Locale::En => "The following processes are locking files. You can kill them and retry, or skip.",
        }
    }

    pub fn conflict_col_pid(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "PID",
            Locale::En => "PID",
        }
    }

    pub fn conflict_col_name(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "프로세스 이름",
            Locale::En => "Process Name",
        }
    }

    pub fn conflict_col_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "실행 경로",
            Locale::En => "Executable Path",
        }
    }

    pub fn btn_kill_and_retry(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "프로세스 종료 후 재시도",
            Locale::En => "Kill & Retry",
        }
    }

    pub fn btn_skip(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "건너뛰기",
            Locale::En => "Skip",
        }
    }

    pub fn log_killing_processes(&self, count: usize) -> String {
        match self.locale {
            Locale::Ko => format!("{}개의 프로세스를 종료합니다...", count),
            Locale::En => format!("Killing {} process(es)...", count),
        }
    }

    pub fn log_retrying(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 재시도 중...", name),
            Locale::En => format!("{}: Retrying...", name),
        }
    }

    pub fn log_skipped(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 건너뛰었습니다.", name),
            Locale::En => format!("{}: Skipped.", name),
        }
    }

    // -- Language selector --
    pub fn language(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "언어",
            Locale::En => "Language",
        }
    }
}
