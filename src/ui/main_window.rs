use std::path::PathBuf;
use std::sync::mpsc;

use eframe::App;
use egui::{Color32, RichText, Vec2};

use crate::core::{RelocationMethod, RelocationTarget, TargetStatus};
use crate::i18n::{Locale, Texts, detect_system_locale};
use crate::mover::{MoveEvent, ProcessInfo, check_conflicting_processes, kill_processes, rollback_target};
use crate::core::format_bytes;
use crate::scanner::{DirSizeEntry, HomeDirSizeResult, SizeResult, scan_home_dirs, scan_sizes_async, scan_targets};
use crate::ui::table::render_target_table;
use crate::utils::disk_usage::{DriveInfo, get_drives};

const COLOR_RESCAN: Color32 = Color32::from_rgb(100, 180, 255);
const COLOR_SET_TARGET: Color32 = Color32::from_rgb(255, 200, 60);
const COLOR_START_MOVE: Color32 = Color32::from_rgb(80, 200, 120);
const COLOR_ROLLBACK: Color32 = Color32::from_rgb(255, 100, 100);

struct ConflictDialog {
    target_index: usize,
    target: RelocationTarget,
    processes: Vec<ProcessInfo>,
    failed_path: String,
}

pub struct MainWindow {
    targets: Vec<RelocationTarget>,
    drives: Vec<DriveInfo>,
    selected_target_base: String,
    size_rx: Option<mpsc::Receiver<SizeResult>>,
    move_rx: Option<mpsc::Receiver<MoveEvent>>,
    dry_run: bool,
    log_messages: Vec<String>,
    process_warnings: Vec<String>,
    texts: Texts,
    conflict_dialog: Option<ConflictDialog>,
    tool_env_vars: Vec<(String, String, String)>,
    home_dirs: Vec<DirSizeEntry>,
    home_dirs_rx: Option<mpsc::Receiver<HomeDirSizeResult>>,
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_korean_font(&cc.egui_ctx);

        let targets = scan_targets();
        let drives = get_drives();
        let locale = detect_system_locale();

        let default_base = drives
            .iter()
            .find(|d| !d.mount_point.starts_with("C:\\") && !d.mount_point.starts_with("c:\\"))
            .map(|d| format!("{}DevHomes", d.mount_point))
            .unwrap_or_else(|| "D:\\DevHomes".to_string());

        let size_rx = start_size_scan(&targets);
        let tool_env_vars = crate::config::collect_tool_env_vars();
        let (home_dirs, home_dirs_rx) = scan_home_dirs();

        Self {
            targets,
            drives,
            selected_target_base: default_base,
            size_rx: Some(size_rx),
            move_rx: None,
            dry_run: false,
            log_messages: Vec::new(),
            process_warnings: Vec::new(),
            texts: Texts::new(locale),
            conflict_dialog: None,
            tool_env_vars,
            home_dirs,
            home_dirs_rx: Some(home_dirs_rx),
        }
    }

    fn rescan(&mut self) {
        self.targets = scan_targets();
        self.drives = get_drives();
        self.size_rx = Some(start_size_scan(&self.targets));
        self.tool_env_vars = crate::config::collect_tool_env_vars();
        let (home_dirs, home_dirs_rx) = scan_home_dirs();
        self.home_dirs = home_dirs;
        self.home_dirs_rx = Some(home_dirs_rx);
        self.log_messages.push(self.texts.log_rescanned().to_string());
    }

    fn has_selected(&self) -> bool {
        self.targets.iter().any(|t| t.enabled)
    }

    fn apply_target_paths(&mut self) {
        if !self.has_selected() {
            self.log_messages.push(self.texts.info_no_selected().to_string());
            return;
        }
        let base = PathBuf::from(&self.selected_target_base);
        for target in &mut self.targets {
            if target.enabled {
                let dir_name = target
                    .current_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                target.target_path = Some(base.join(&dir_name));
                target.status = TargetStatus::Configured;
            }
        }
        self.log_messages
            .push(self.texts.log_targets_configured().to_string());
    }

    fn move_selected(&mut self) {
        if !self.has_selected() {
            self.log_messages.push(self.texts.info_no_selected().to_string());
            return;
        }
        self.process_warnings = check_conflicting_processes();
        if !self.process_warnings.is_empty() && !self.dry_run {
            let msg = self.texts.log_process_warning(&self.process_warnings);
            self.log_messages.push(msg);
        }

        let enabled: Vec<(usize, RelocationTarget)> = self
            .targets
            .iter()
            .enumerate()
            .filter(|(_, t)| t.enabled && t.target_path.is_some())
            .map(|(i, t)| (i, t.clone()))
            .collect();

        if enabled.is_empty() {
            self.log_messages
                .push(self.texts.log_no_selection().to_string());
            return;
        }

        if self.dry_run {
            for (_, target) in &enabled {
                let target_display = target.target_path.as_ref().unwrap().display().to_string();
                self.log_messages.push(self.texts.log_dry_run_move(
                    &target.name,
                    &target.size_display(self.texts.scanning()),
                    &target_display,
                ));
                if let RelocationMethod::EnvVar { ref var_name } = target.method {
                    self.log_messages
                        .push(self.texts.log_dry_run_env(var_name, &target_display));
                }
            }
            return;
        }

        for (i, _) in &enabled {
            self.targets[*i].status = TargetStatus::Moving;
        }

        let rx = crate::mover::move_targets_async(enabled);
        self.move_rx = Some(rx);
    }

    fn rollback_selected(&mut self) {
        if !self.has_selected() {
            self.log_messages.push(self.texts.info_no_selected().to_string());
            return;
        }
        for target in &mut self.targets {
            if !target.enabled {
                continue;
            }
            match rollback_target(target) {
                Ok(()) => {
                    let msg = self.texts.log_rolled_back(&target.name);
                    self.log_messages.push(msg);
                    target.status = TargetStatus::Rolledback;
                }
                Err(e) => {
                    let msg = self.texts.log_rollback_failed(&target.name, &e);
                    self.log_messages.push(msg);
                    target.status = TargetStatus::Failed(e);
                }
            }
        }
    }

    fn poll_size_results(&mut self) {
        if let Some(ref rx) = self.size_rx {
            while let Ok(result) = rx.try_recv() {
                if result.index < self.targets.len() {
                    self.targets[result.index].size_bytes = Some(result.size_bytes);
                }
            }
        }
        if let Some(ref rx) = self.home_dirs_rx {
            while let Ok(result) = rx.try_recv() {
                if result.index < self.home_dirs.len() {
                    self.home_dirs[result.index].size_bytes = Some(result.size_bytes);
                    self.home_dirs[result.index].is_scanning = false;
                }
            }
        }
    }

    fn poll_move_results(&mut self) {
        if let Some(ref rx) = self.move_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    MoveEvent::Progress { index, percent } => {
                        if index < self.targets.len() {
                            self.targets[index].progress = percent;
                        }
                    }
                    MoveEvent::Completed { index } => {
                        self.targets[index].progress = 100.0;
                        self.targets[index].status = TargetStatus::Moved;
                        let msg =
                            self.texts.log_migration_complete(&self.targets[index].name);
                        self.log_messages.push(msg);
                    }
                    MoveEvent::Failed { index, reason } => {
                        self.targets[index].status = TargetStatus::Failed(reason.clone());
                        let msg = self
                            .texts
                            .log_migration_failed(&self.targets[index].name, &reason);
                        self.log_messages.push(msg);
                    }
                    MoveEvent::ProcessConflict {
                        index,
                        failed_path,
                        processes,
                    } => {
                        self.targets[index].status =
                            TargetStatus::Failed("File locked".to_string());
                        self.conflict_dialog = Some(ConflictDialog {
                            target_index: index,
                            target: self.targets[index].clone(),
                            processes,
                            failed_path,
                        });
                    }
                }
            }
        }
    }
}

fn setup_korean_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Try to load Malgun Gothic (맑은 고딕) from Windows fonts
    let font_paths = [
        "C:\\Windows\\Fonts\\malgun.ttf",
        "C:\\Windows\\Fonts\\malgungsl.ttf",
    ];

    for path in &font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "korean_font".to_owned(),
                egui::FontData::from_owned(font_data).into(),
            );

            // Put Korean font first for proportional text
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "korean_font".to_owned());

            // Also for monospace
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "korean_font".to_owned());

            break;
        }
    }

    ctx.set_fonts(fonts);
}

fn colored_button(ui: &mut egui::Ui, label: &str, color: Color32) -> bool {
    colored_button_with_tooltip(ui, label, color, None)
}

fn colored_button_with_tooltip(ui: &mut egui::Ui, label: &str, color: Color32, tooltip: Option<&str>) -> bool {
    let button = egui::Button::new(
        RichText::new(label).color(Color32::BLACK).strong(),
    )
    .fill(color)
    .min_size(Vec2::new(100.0, 28.0));
    let response = ui.add(button);
    if let Some(tip) = tooltip {
        response.clone().on_hover_text(tip);
    }
    response.clicked()
}

fn start_size_scan(targets: &[RelocationTarget]) -> mpsc::Receiver<SizeResult> {
    let paths: Vec<(usize, PathBuf)> = targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            // For already-moved targets, scan the actual relocated path
            if t.status == TargetStatus::AlreadyMoved {
                if let Some(ref tp) = t.target_path {
                    return (i, tp.clone());
                }
            }
            (i, t.current_path.clone())
        })
        .collect();
    scan_sizes_async(paths)
}

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_size_results();
        self.poll_move_results();

        if self.size_rx.is_some() || self.move_rx.is_some() || self.home_dirs_rx.is_some() {
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.texts.app_title());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(self.texts.language());
                    let current = self.texts.locale;
                    for locale in [Locale::Ko, Locale::En, Locale::Ja, Locale::Zh] {
                        if ui
                            .selectable_label(current == locale, locale.label())
                            .clicked()
                        {
                            self.texts = Texts::new(locale);
                        }
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                let home = dirs::home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                ui.label(format!("{}: {}", self.texts.home_directory(), home));

                ui.separator();

                for drive in &self.drives {
                    ui.label(format!(
                        "{} {}: {} / {}",
                        drive.mount_point,
                        self.texts.free_space_fmt(),
                        drive.free_display(),
                        drive.total_display()
                    ));
                }
            });
        });

        egui::TopBottomPanel::bottom("log_panel")
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.heading(self.texts.log_title());
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for msg in &self.log_messages {
                            ui.label(msg);
                        }
                    });
            });

        // Process conflict dialog
        let mut conflict_action: Option<bool> = None; // Some(true) = kill, Some(false) = skip
        if let Some(ref dialog) = self.conflict_dialog {
            egui::Window::new(self.texts.conflict_dialog_title())
                .collapsible(false)
                .resizable(true)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(self.texts.conflict_dialog_desc());
                    ui.add_space(8.0);

                    ui.label(
                        RichText::new(&dialog.failed_path)
                            .color(Color32::LIGHT_RED)
                            .small(),
                    );
                    ui.add_space(8.0);

                    egui::Grid::new("conflict_process_table")
                        .num_columns(3)
                        .striped(true)
                        .spacing([12.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong(self.texts.conflict_col_pid());
                            ui.strong(self.texts.conflict_col_name());
                            ui.strong(self.texts.conflict_col_path());
                            ui.end_row();

                            for p in &dialog.processes {
                                ui.label(p.pid.to_string());
                                ui.label(&p.name);
                                ui.label(&p.exe_path);
                                ui.end_row();
                            }
                        });

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if colored_button(ui, self.texts.btn_kill_and_retry(), COLOR_ROLLBACK) {
                            conflict_action = Some(true);
                        }
                        ui.add_space(8.0);
                        if colored_button(ui, self.texts.btn_skip(), Color32::GRAY) {
                            conflict_action = Some(false);
                        }
                    });
                });
        }

        if let Some(kill) = conflict_action {
            if let Some(dialog) = self.conflict_dialog.take() {
                if kill {
                    let pids: Vec<u32> = dialog.processes.iter().map(|p| p.pid).collect();
                    self.log_messages
                        .push(self.texts.log_killing_processes(pids.len()));
                    kill_processes(&pids);

                    // Retry the move for this target
                    self.log_messages
                        .push(self.texts.log_retrying(&dialog.target.name));
                    self.targets[dialog.target_index].status = TargetStatus::Moving;
                    let rx = crate::mover::move_targets_async(vec![(
                        dialog.target_index,
                        dialog.target,
                    )]);
                    self.move_rx = Some(rx);
                } else {
                    self.log_messages
                        .push(self.texts.log_skipped(&dialog.target.name));
                }
            }
        }

        // Left panel: home directory sizes
        egui::SidePanel::left("home_dirs_panel")
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading(self.texts.home_dirs_title());
                ui.separator();

                // Sort by size descending for display
                let mut sorted_indices: Vec<usize> = (0..self.home_dirs.len()).collect();
                sorted_indices.sort_by(|&a, &b| {
                    let sa = self.home_dirs[a].size_bytes.unwrap_or(0);
                    let sb = self.home_dirs[b].size_bytes.unwrap_or(0);
                    sb.cmp(&sa)
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("home_dirs_grid")
                        .num_columns(2)
                        .striped(true)
                        .spacing([8.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong(self.texts.home_dirs_col_name());
                            ui.strong(self.texts.home_dirs_col_size());
                            ui.end_row();

                            for &i in &sorted_indices {
                                let entry = &self.home_dirs[i];
                                let size_text = if entry.is_scanning {
                                    self.texts.scanning().to_string()
                                } else {
                                    match entry.size_bytes {
                                        Some(b) => format_bytes(b),
                                        None => "—".to_string(),
                                    }
                                };

                                // Color by size
                                let color = match entry.size_bytes {
                                    Some(b) if b >= 1024 * 1024 * 1024 => Color32::LIGHT_RED,
                                    Some(b) if b >= 100 * 1024 * 1024 => Color32::YELLOW,
                                    Some(b) if b >= 10 * 1024 * 1024 => Color32::LIGHT_GREEN,
                                    _ => Color32::GRAY,
                                };

                                let is_junction = crate::mover::junction::is_junction(&entry.path);
                                if is_junction {
                                    ui.label(RichText::new(&entry.name).strikethrough());
                                } else {
                                    ui.label(&entry.name);
                                }
                                ui.label(RichText::new(&size_text).color(color));
                                ui.end_row();
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(self.texts.target_base_directory());
                ui.text_edit_singleline(&mut self.selected_target_base);
                if ui.button(self.texts.browse()).clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.selected_target_base = folder.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.dry_run, self.texts.dry_run());
            });

            ui.separator();

            ui.horizontal(|ui| {
                if colored_button_with_tooltip(ui, self.texts.btn_rescan(), COLOR_RESCAN, Some(self.texts.tooltip_rescan())) {
                    self.rescan();
                }
                if colored_button_with_tooltip(ui, self.texts.btn_set_target(), COLOR_SET_TARGET, Some(self.texts.tooltip_set_target())) {
                    self.apply_target_paths();
                }
                if colored_button_with_tooltip(ui, self.texts.btn_start_move(), COLOR_START_MOVE, Some(self.texts.tooltip_start_move())) {
                    self.move_selected();
                }
                ui.add_space(20.0);
                if colored_button_with_tooltip(ui, self.texts.btn_rollback(), COLOR_ROLLBACK, Some(self.texts.tooltip_rollback())) {
                    self.rollback_selected();
                }
            });

            ui.separator();

            if !self.process_warnings.is_empty() {
                ui.colored_label(
                    Color32::YELLOW,
                    self.texts
                        .process_warning(&self.process_warnings.join(", ")),
                );
                ui.separator();
            }

            egui::ScrollArea::both().show(ui, |ui| {
                render_target_table(ui, &mut self.targets, &self.texts);

                if !self.tool_env_vars.is_empty() {
                    ui.add_space(12.0);
                    egui::CollapsingHeader::new(
                        RichText::new(self.texts.tool_env_vars_title()).strong(),
                    )
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("tool_env_vars_table")
                            .num_columns(3)
                            .striped(true)
                            .spacing([12.0, 4.0])
                            .show(ui, |ui| {
                                ui.strong(self.texts.env_col_variable());
                                ui.strong(self.texts.env_col_value());
                                ui.strong(self.texts.env_col_scope());
                                ui.end_row();

                                for (name, value, scope) in &self.tool_env_vars {
                                    ui.label(
                                        RichText::new(name).color(Color32::LIGHT_BLUE),
                                    );
                                    ui.label(value);
                                    ui.label(
                                        RichText::new(scope).color(Color32::GRAY),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        });
    }
}
