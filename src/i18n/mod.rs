#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    Ko,
    En,
    Ja,
    Zh,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::Ko => "한국어",
            Locale::En => "English",
            Locale::Ja => "日本語",
            Locale::Zh => "中文",
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
    if lang.starts_with("ja") {
        return Locale::Ja;
    }
    if lang.starts_with("zh") {
        return Locale::Zh;
    }

    // Windows: check user locale via registry
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("Control Panel\\International", KEY_READ)
        {
            if let Ok(locale_name) = key.get_value::<String, _>("LocaleName") {
                let ln = locale_name.to_lowercase();
                if ln.starts_with("ko") {
                    return Locale::Ko;
                }
                if ln.starts_with("ja") {
                    return Locale::Ja;
                }
                if ln.starts_with("zh") {
                    return Locale::Zh;
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
            Locale::Ko => "DevHome Relocator - 개발 캐시 이동 도구 (Built with Rust + egui)",
            Locale::En => "DevHome Relocator (Built with Rust + egui)",
            Locale::Ja => "DevHome Relocator - 開発キャッシュ移動ツール (Built with Rust + egui)",
            Locale::Zh => "DevHome Relocator - 开发缓存迁移工具 (Built with Rust + egui)",
        }
    }

    // -- Header --
    pub fn home_directory(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "홈 디렉토리",
            Locale::En => "Home Directory",
            Locale::Ja => "ホームディレクトリ",
            Locale::Zh => "主目录",
        }
    }

    pub fn free_space_fmt(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "여유",
            Locale::En => "Free",
            Locale::Ja => "空き",
            Locale::Zh => "可用",
        }
    }

    // -- Target base selector --
    pub fn target_base_directory(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동할 위치:",
            Locale::En => "Target directory:",
            Locale::Ja => "移動先:",
            Locale::Zh => "目标目录:",
        }
    }

    pub fn browse(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "찾아보기...",
            Locale::En => "Browse...",
            Locale::Ja => "参照...",
            Locale::Zh => "浏览...",
        }
    }

    pub fn dry_run(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "시뮬레이션 모드 (실제 변경 없음)",
            Locale::En => "Dry Run (no actual changes)",
            Locale::Ja => "シミュレーションモード（実際の変更なし）",
            Locale::Zh => "模拟模式（不会实际更改）",
        }
    }

    // -- Table headers --
    pub fn col_select(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "선택",
            Locale::En => "Select",
            Locale::Ja => "選択",
            Locale::Zh => "选择",
        }
    }

    pub fn col_tool(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "도구",
            Locale::En => "Tool",
            Locale::Ja => "ツール",
            Locale::Zh => "工具",
        }
    }

    pub fn col_current_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "현재 경로",
            Locale::En => "Current Path",
            Locale::Ja => "現在のパス",
            Locale::Zh => "当前路径",
        }
    }

    pub fn col_size(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "크기",
            Locale::En => "Size",
            Locale::Ja => "サイズ",
            Locale::Zh => "大小",
        }
    }

    pub fn col_target_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 경로",
            Locale::En => "Target Path",
            Locale::Ja => "移動先パス",
            Locale::Zh => "目标路径",
        }
    }

    pub fn col_method(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "방식",
            Locale::En => "Method",
            Locale::Ja => "方式",
            Locale::Zh => "方式",
        }
    }

    pub fn col_env_var(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "환경변수",
            Locale::En => "Env Variable",
            Locale::Ja => "環境変数",
            Locale::Zh => "环境变量",
        }
    }

    pub fn col_status(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "상태",
            Locale::En => "Status",
            Locale::Ja => "状態",
            Locale::Zh => "状态",
        }
    }

    pub fn env_not_set(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "(미설정)",
            Locale::En => "(not set)",
            Locale::Ja => "（未設定）",
            Locale::Zh => "（未设置）",
        }
    }

    // -- Status labels --
    pub fn status_detected(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "발견됨",
            Locale::En => "Detected",
            Locale::Ja => "検出済み",
            Locale::Zh => "已检测",
        }
    }

    pub fn status_already_moved(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동됨 (설정 완료)",
            Locale::En => "Already Moved",
            Locale::Ja => "移動済み（設定完了）",
            Locale::Zh => "已迁移（设置完成）",
        }
    }

    pub fn status_configured(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "설정 완료",
            Locale::En => "Configured",
            Locale::Ja => "設定完了",
            Locale::Zh => "已配置",
        }
    }

    pub fn status_moved(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 완료",
            Locale::En => "Moved",
            Locale::Ja => "移動完了",
            Locale::Zh => "迁移完成",
        }
    }

    pub fn status_failed(&self, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("실패: {}", reason),
            Locale::En => format!("Failed: {}", reason),
            Locale::Ja => format!("失敗: {}", reason),
            Locale::Zh => format!("失败: {}", reason),
        }
    }

    pub fn status_rolledback(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "복원됨",
            Locale::En => "Rolled back",
            Locale::Ja => "復元済み",
            Locale::Zh => "已回滚",
        }
    }

    // -- Scanning --
    pub fn scanning(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "분석 중...",
            Locale::En => "Scanning...",
            Locale::Ja => "分析中...",
            Locale::Zh => "扫描中...",
        }
    }

    // -- Buttons --
    pub fn btn_rescan(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "다시 검색",
            Locale::En => "Rescan",
            Locale::Ja => "再スキャン",
            Locale::Zh => "重新扫描",
        }
    }

    pub fn btn_set_target(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "경로 지정",
            Locale::En => "Set Target",
            Locale::Ja => "パス設定",
            Locale::Zh => "设置路径",
        }
    }

    pub fn btn_start_move(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 시작",
            Locale::En => "Start Move",
            Locale::Ja => "移動開始",
            Locale::Zh => "开始迁移",
        }
    }

    pub fn btn_rollback(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "되돌리기",
            Locale::En => "Undo / Rollback",
            Locale::Ja => "元に戻す",
            Locale::Zh => "回滚",
        }
    }

    // -- Log panel --
    pub fn log_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "실행 로그",
            Locale::En => "Activity Log",
            Locale::Ja => "実行ログ",
            Locale::Zh => "执行日志",
        }
    }

    // -- Log messages --
    pub fn log_rescanned(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "디렉토리를 다시 검색했습니다.",
            Locale::En => "Directories rescanned.",
            Locale::Ja => "ディレクトリを再スキャンしました。",
            Locale::Zh => "已重新扫描目录。",
        }
    }

    pub fn log_targets_configured(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동 경로가 설정되었습니다.",
            Locale::En => "Target paths configured.",
            Locale::Ja => "移動先パスが設定されました。",
            Locale::Zh => "目标路径已配置。",
        }
    }

    pub fn log_no_selection(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "이동할 항목을 선택해주세요.",
            Locale::En => "No targets selected for migration.",
            Locale::Ja => "移動する項目を選択してください。",
            Locale::Zh => "请选择要迁移的项目。",
        }
    }

    pub fn info_no_selected(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "⚠ 선택된 항목이 없습니다. 테이블에서 항목을 선택해주세요.",
            Locale::En => "⚠ No items selected. Please select items from the table.",
            Locale::Ja => "⚠ 選択された項目がありません。テーブルから項目を選択してください。",
            Locale::Zh => "⚠ 未选择任何项目。请从表格中选择项目。",
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
            Locale::Ja => format!(
                "警告: 関連プロセスが実行中です: {}",
                processes.join(", ")
            ),
            Locale::Zh => format!(
                "警告: 检测到冲突进程: {}",
                processes.join(", ")
            ),
        }
    }

    pub fn log_dry_run_move(&self, name: &str, size: &str, target: &str) -> String {
        match self.locale {
            Locale::Ko => format!("[시뮬레이션] {} ({}) -> {}", name, size, target),
            Locale::En => format!("[DRY RUN] Would move {} ({}) -> {}", name, size, target),
            Locale::Ja => format!("[シミュレーション] {} ({}) -> {}", name, size, target),
            Locale::Zh => format!("[模拟] {} ({}) -> {}", name, size, target),
        }
    }

    pub fn log_dry_run_env(&self, var_name: &str, value: &str) -> String {
        match self.locale {
            Locale::Ko => format!("[시뮬레이션] {} = {} 설정 예정", var_name, value),
            Locale::En => format!("[DRY RUN] Would set {} = {}", var_name, value),
            Locale::Ja => format!("[シミュレーション] {} = {} を設定予定", var_name, value),
            Locale::Zh => format!("[模拟] 将设置 {} = {}", var_name, value),
        }
    }

    pub fn log_migration_complete(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 이동 완료.", name),
            Locale::En => format!("{}: Migration complete.", name),
            Locale::Ja => format!("{}: 移動完了。", name),
            Locale::Zh => format!("{}: 迁移完成。", name),
        }
    }

    pub fn log_migration_failed(&self, name: &str, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 실패 - {}", name, reason),
            Locale::En => format!("{}: Failed - {}", name, reason),
            Locale::Ja => format!("{}: 失敗 - {}", name, reason),
            Locale::Zh => format!("{}: 失败 - {}", name, reason),
        }
    }

    pub fn log_rolled_back(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 복원되었습니다.", name),
            Locale::En => format!("{}: Rolled back.", name),
            Locale::Ja => format!("{}: 復元されました。", name),
            Locale::Zh => format!("{}: 已回滚。", name),
        }
    }

    pub fn log_rollback_failed(&self, name: &str, reason: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 복원 실패 - {}", name, reason),
            Locale::En => format!("{}: Rollback failed - {}", name, reason),
            Locale::Ja => format!("{}: 復元失敗 - {}", name, reason),
            Locale::Zh => format!("{}: 回滚失败 - {}", name, reason),
        }
    }

    // -- Process warning --
    pub fn process_warning(&self, processes: &str) -> String {
        match self.locale {
            Locale::Ko => format!("경고: 실행 중인 프로세스가 있습니다: {}", processes),
            Locale::En => format!("Warning: active processes detected: {}", processes),
            Locale::Ja => format!("警告: 実行中のプロセスがあります: {}", processes),
            Locale::Zh => format!("警告: 检测到活动进程: {}", processes),
        }
    }

    // -- Process conflict dialog --
    pub fn conflict_dialog_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "파일 잠금 충돌",
            Locale::En => "File Lock Conflict",
            Locale::Ja => "ファイルロック競合",
            Locale::Zh => "文件锁定冲突",
        }
    }

    pub fn conflict_dialog_desc(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "다음 프로세스가 파일을 사용 중입니다. 종료 후 재시도하거나 건너뛸 수 있습니다.",
            Locale::En => "The following processes are locking files. You can kill them and retry, or skip.",
            Locale::Ja => "以下のプロセスがファイルを使用中です。終了して再試行するか、スキップできます。",
            Locale::Zh => "以下进程正在使用文件。您可以终止它们并重试，或跳过。",
        }
    }

    pub fn conflict_col_pid(&self) -> &'static str {
        match self.locale {
            Locale::Ko | Locale::En | Locale::Ja | Locale::Zh => "PID",
        }
    }

    pub fn conflict_col_name(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "프로세스 이름",
            Locale::En => "Process Name",
            Locale::Ja => "プロセス名",
            Locale::Zh => "进程名称",
        }
    }

    pub fn conflict_col_path(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "실행 경로",
            Locale::En => "Executable Path",
            Locale::Ja => "実行パス",
            Locale::Zh => "可执行路径",
        }
    }

    pub fn btn_kill_and_retry(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "프로세스 종료 후 재시도",
            Locale::En => "Kill & Retry",
            Locale::Ja => "プロセス終了後に再試行",
            Locale::Zh => "终止进程并重试",
        }
    }

    pub fn btn_skip(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "건너뛰기",
            Locale::En => "Skip",
            Locale::Ja => "スキップ",
            Locale::Zh => "跳过",
        }
    }

    pub fn log_killing_processes(&self, count: usize) -> String {
        match self.locale {
            Locale::Ko => format!("{}개의 프로세스를 종료합니다...", count),
            Locale::En => format!("Killing {} process(es)...", count),
            Locale::Ja => format!("{}個のプロセスを終了しています...", count),
            Locale::Zh => format!("正在终止 {} 个进程...", count),
        }
    }

    pub fn log_retrying(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 재시도 중...", name),
            Locale::En => format!("{}: Retrying...", name),
            Locale::Ja => format!("{}: 再試行中...", name),
            Locale::Zh => format!("{}: 正在重试...", name),
        }
    }

    pub fn log_skipped(&self, name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("{}: 건너뛰었습니다.", name),
            Locale::En => format!("{}: Skipped.", name),
            Locale::Ja => format!("{}: スキップしました。", name),
            Locale::Zh => format!("{}: 已跳过。", name),
        }
    }

    // -- Button tooltips --
    pub fn tooltip_rescan(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "시스템을 다시 스캔하여 이동 가능한 디렉토리 목록과\n드라이브 정보, 환경변수를 갱신합니다.",
            Locale::En => "Rescan the system to refresh the list of relocatable\ndirectories, drive info, and environment variables.",
            Locale::Ja => "システムを再スキャンして、移動可能なディレクトリ一覧、\nドライブ情報、環境変数を更新します。",
            Locale::Zh => "重新扫描系统，刷新可迁移目录列表、\n驱动器信息和环境变量。",
        }
    }

    pub fn tooltip_set_target(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "선택된 항목의 이동 대상 경로를 위의 기본 경로 기준으로\n자동 설정합니다. (예: D:\\DevHomes\\.cargo)",
            Locale::En => "Set the target path for selected items based on\nthe base directory above. (e.g. D:\\DevHomes\\.cargo)",
            Locale::Ja => "選択した項目の移動先パスを上の基本パスに基づいて\n自動設定します。（例: D:\\DevHomes\\.cargo）",
            Locale::Zh => "根据上方的基本路径自动设置所选项目的\n目标路径。（例: D:\\DevHomes\\.cargo）",
        }
    }

    pub fn tooltip_start_move(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "선택된 항목을 대상 경로로 복사 후 원본을 삭제하고,\n환경변수 설정 또는 Junction 링크를 생성합니다.",
            Locale::En => "Copy selected items to the target path, remove originals,\nand set environment variables or create junction links.",
            Locale::Ja => "選択した項目を移動先にコピー後、元のファイルを削除し、\n環境変数の設定またはJunctionリンクを作成します。",
            Locale::Zh => "将所选项目复制到目标路径，删除原始文件，\n并设置环境变量或创建Junction链接。",
        }
    }

    pub fn tooltip_rollback(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "선택된 항목의 이동을 되돌립니다.\n대상 경로에서 원래 경로로 파일을 복원하고\n환경변수/Junction 설정을 원래 상태로 되돌립니다.",
            Locale::En => "Rollback selected items.\nRestore files from target path to original path\nand revert environment variables/junction settings.",
            Locale::Ja => "選択した項目の移動を元に戻します。\n移動先から元のパスにファイルを復元し、\n環境変数/Junction設定を元の状態に戻します。",
            Locale::Zh => "回滚所选项目。\n从目标路径恢复文件到原始路径，\n并还原环境变量/Junction设置。",
        }
    }

    // -- Method tooltips --
    pub fn tooltip_junction(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "Junction: NTFS 디렉토리 연결입니다.\n원래 경로에 가상 링크를 만들어 실제 파일은 다른 드라이브에 저장합니다.\n프로그램은 원래 경로를 그대로 사용하므로 설정 변경이 필요 없습니다.",
            Locale::En => "Junction: An NTFS directory link.\nCreates a virtual link at the original path pointing to the new drive.\nPrograms use the original path transparently — no config changes needed.",
            Locale::Ja => "Junction: NTFSディレクトリリンクです。\n元のパスに仮想リンクを作成し、実際のファイルは別のドライブに保存します。\nプログラムは元のパスをそのまま使用するため、設定変更は不要です。",
            Locale::Zh => "Junction: NTFS目录链接。\n在原始路径创建虚拟链接，实际文件存储在其他驱动器。\n程序透明使用原始路径，无需更改配置。",
        }
    }

    pub fn tooltip_envvar(&self, var_name: &str) -> String {
        match self.locale {
            Locale::Ko => format!("EnvVar: 환경변수 방식입니다.\n{} 환경변수를 새 경로로 설정하여\n프로그램이 변경된 위치를 직접 참조하도록 합니다.", var_name),
            Locale::En => format!("EnvVar: Environment variable method.\nSets the {} variable to the new path\nso the program directly references the new location.", var_name),
            Locale::Ja => format!("EnvVar: 環境変数方式です。\n{} 環境変数を新しいパスに設定し、\nプログラムが変更された場所を直接参照するようにします。", var_name),
            Locale::Zh => format!("EnvVar: 环境变量方式。\n将 {} 环境变量设置为新路径，\n使程序直接引用新位置。", var_name),
        }
    }

    // -- Tool environment variables panel --
    pub fn tool_env_vars_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "도구 환경변수 (바이너리 경로)",
            Locale::En => "Tool Environment Variables (Binary Paths)",
            Locale::Ja => "ツール環境変数（バイナリパス）",
            Locale::Zh => "工具环境变量（二进制路径）",
        }
    }

    pub fn env_col_variable(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "변수명",
            Locale::En => "Variable",
            Locale::Ja => "変数名",
            Locale::Zh => "变量名",
        }
    }

    pub fn env_col_value(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "값",
            Locale::En => "Value",
            Locale::Ja => "値",
            Locale::Zh => "值",
        }
    }

    pub fn env_col_scope(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "범위",
            Locale::En => "Scope",
            Locale::Ja => "スコープ",
            Locale::Zh => "范围",
        }
    }

    // -- Home directory size panel --
    pub fn home_dirs_title(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "홈 디렉토리 사용량",
            Locale::En => "Home Directory Usage",
            Locale::Ja => "ホームディレクトリ使用量",
            Locale::Zh => "主目录使用量",
        }
    }

    pub fn home_dirs_col_name(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "디렉토리",
            Locale::En => "Directory",
            Locale::Ja => "ディレクトリ",
            Locale::Zh => "目录",
        }
    }

    pub fn home_dirs_col_size(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "크기",
            Locale::En => "Size",
            Locale::Ja => "サイズ",
            Locale::Zh => "大小",
        }
    }

    // -- Language selector --
    pub fn language(&self) -> &'static str {
        match self.locale {
            Locale::Ko => "언어",
            Locale::En => "Language",
            Locale::Ja => "言語",
            Locale::Zh => "语言",
        }
    }
}
