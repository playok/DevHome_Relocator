use egui::{Color32, RichText, Ui};

use crate::core::{RelocationMethod, RelocationTarget, TargetStatus};
use crate::i18n::Texts;

pub fn render_target_table(ui: &mut Ui, targets: &mut Vec<RelocationTarget>, texts: &Texts) {
    egui::Grid::new("target_table")
        .num_columns(8)
        .striped(true)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.strong(texts.col_select());
            ui.strong(texts.col_tool());
            ui.strong(texts.col_current_path());
            ui.strong(texts.col_size());
            ui.strong(texts.col_target_path());
            ui.strong(texts.col_method());
            ui.strong(texts.col_env_var());
            ui.strong(texts.col_status());
            ui.end_row();

            for target in targets.iter_mut() {
                ui.checkbox(&mut target.enabled, "");
                ui.label(&target.name);
                ui.label(target.current_path.to_string_lossy().as_ref());
                ui.label(target.size_display(texts.scanning()));

                if let Some(ref tp) = target.target_path {
                    ui.label(tp.to_string_lossy().as_ref());
                } else {
                    ui.label("—");
                }

                ui.label(target.method.to_string());

                // Environment variable column
                match &target.method {
                    RelocationMethod::EnvVar { var_name } => {
                        if let Some(ref val) = target.env_current_value {
                            ui.label(
                                RichText::new(format!("{} = {}", var_name, val))
                                    .color(Color32::LIGHT_GREEN),
                            );
                        } else {
                            ui.label(
                                RichText::new(format!("{} {}", var_name, texts.env_not_set()))
                                    .color(Color32::GRAY),
                            );
                        }
                    }
                    RelocationMethod::Junction => {
                        ui.label(RichText::new("—").color(Color32::GRAY));
                    }
                }

                let status_text = match &target.status {
                    TargetStatus::Detected => {
                        RichText::new(texts.status_detected()).color(Color32::LIGHT_BLUE)
                    }
                    TargetStatus::AlreadyMoved => {
                        RichText::new(texts.status_already_moved())
                            .color(Color32::from_rgb(140, 220, 255))
                    }
                    TargetStatus::Configured => {
                        RichText::new(texts.status_configured()).color(Color32::YELLOW)
                    }
                    TargetStatus::Moving => {
                        RichText::new(texts.status_moving()).color(Color32::GOLD)
                    }
                    TargetStatus::Moved => {
                        RichText::new(texts.status_moved()).color(Color32::LIGHT_GREEN)
                    }
                    TargetStatus::Failed(reason) => {
                        RichText::new(texts.status_failed(reason)).color(Color32::LIGHT_RED)
                    }
                    TargetStatus::Rolledback => {
                        RichText::new(texts.status_rolledback()).color(Color32::GRAY)
                    }
                };
                ui.label(status_text);

                ui.end_row();
            }
        });
}
