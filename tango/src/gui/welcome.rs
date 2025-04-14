use crate::{config, game, gui, i18n};
use fluent_templates::Loader;

pub struct State {
    nickname: String,
    done_inputting_roms: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            nickname: "".to_string(),
            done_inputting_roms: false,
        }
    }
}

pub fn show(
    ctx: &egui::Context,
    shared_root_state: &gui::SharedRootState,
    config: &mut config::Config,
    state: &mut State,
) {
    let roms_scanner = &shared_root_state.roms_scanner;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(8.0);
            let emblem = egui::Image::new(egui::include_image!("../emblem.png"));
            let emblem_size = emblem.load_and_calc_size(ui, egui::Vec2::INFINITY).unwrap_or_default() * 0.5;
            ui.add(emblem.fit_to_exact_size(emblem_size));

            ui.add_space(8.0);
            ui.add(egui::Separator::default().vertical());
            ui.add_space(8.0);

            let has_roms = !roms_scanner.read().is_empty();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        gui::language_select::show(ui, &shared_root_state.font_families, &mut config.language);
                    });
                });

                ui.add_space(16.0);
                ui.vertical(|ui| {
                    ui.heading(i18n::LOCALES.lookup(&config.language, "welcome-heading").unwrap());
                    ui.label(i18n::LOCALES.lookup(&config.language, "welcome-description").unwrap());

                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        if has_roms {
                            ui.label(egui::RichText::new("✅").color(egui::Color32::from_rgb(0x4c, 0xaf, 0x50)));
                        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) || cfg!(all(target_os = "linux", target_arch = "x86_64")) || cfg!(target_os = "macos") {
                            ui.label(egui::RichText::new("⌛").color(egui::Color32::from_rgb(0xf4, 0xba, 0x51)));
                        } else {
                            ui.label(egui::RichText::new("❌").color(egui::Color32::from_rgb(0xff, 0x0, 0x0)));
                        }
                        ui.strong(i18n::LOCALES.lookup(&config.language, "welcome-step-1").unwrap());
                    });
                    if !has_roms {
                        if cfg!(all(target_os = "windows", target_arch = "x86_64")) || cfg!(all(target_os = "linux", target_arch = "x86_64")) || cfg!(target_os = "macos") {   
                        ui.label({
                            i18n::LOCALES
                                .lookup(&config.language, "welcome-step-1-description")
                                .unwrap()
                        });
                        } else {
                            ui.strong(i18n::LOCALES.lookup(&config.language, "welcome-step-1-unsupported-platform").unwrap());
                            ui.hyperlink_to(i18n::LOCALES.lookup(&config.language, "welcome-step-1-see-here").unwrap(), "https://github.com/HikariCalyx/trill/wiki/How-to-get-game-ROM-images-legally");
                        }
                    }

                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        if state.done_inputting_roms {
                            ui.label(egui::RichText::new("✅").color(egui::Color32::from_rgb(0x4c, 0xaf, 0x50)));
                        } else {
                            ui.label(egui::RichText::new("⌛").color(egui::Color32::from_rgb(0xf4, 0xba, 0x51)));
                        }
                        ui.strong(i18n::LOCALES.lookup(&config.language, "welcome-step-1-roms").unwrap());
                    });

                    if !state.done_inputting_roms {
                        ui.label({
                            i18n::LOCALES
                                .lookup(&config.language, "welcome-step-1-description-roms")
                                .unwrap()
                        });

                        ui.monospace(format!("{}", config.roms_path().display()));

                        ui.horizontal(|ui| {
                            if ui
                                .button(i18n::LOCALES.lookup(&config.language, "welcome-open-folder").unwrap())
                                .clicked()
                            {
                                let _ = open::that(config.roms_path());
                            }

                            ui.add_enabled_ui(!roms_scanner.is_scanning(), |ui| {
                                if ui
                                    .button(i18n::LOCALES.lookup(&config.language, "welcome-continue").unwrap())
                                    .clicked()
                                {
                                    let roms_path = config.roms_path();
                                    let cloned_roms_scanner = roms_scanner.clone();
                                    let egui_ctx = ui.ctx().clone();
                                    tokio::task::spawn_blocking(move || {
                                        cloned_roms_scanner.rescan(|| Some(game::scan_roms(&roms_path)));
                                        egui_ctx.request_repaint();
                                    });

                                    state.done_inputting_roms = true;
                                }
                            });
                        });
                    }
                });

                ui.add_space(16.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("⌛").color(egui::Color32::from_rgb(0xf4, 0xba, 0x51)));
                        ui.strong(i18n::LOCALES.lookup(&config.language, "welcome-step-3").unwrap());
                    });
                    if has_roms {
                        ui.label(
                            i18n::LOCALES
                                .lookup(&config.language, "welcome-step-3-description")
                                .unwrap(),
                        );
                        ui.horizontal(|ui| {
                            let mut submitted = false;
                            let input_resp = ui.add(
                                egui::TextEdit::singleline(&mut state.nickname)
                                    .hint_text(i18n::LOCALES.lookup(&config.language, "settings-nickname").unwrap())
                                    .desired_width(200.0),
                            );
                            if input_resp.lost_focus() && ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                                submitted = true;
                            }
                            state.nickname = state.nickname.chars().take(20).collect::<String>().trim().to_string();

                            if ui
                                .button(i18n::LOCALES.lookup(&config.language, "welcome-continue").unwrap())
                                .clicked()
                            {
                                submitted = true;
                            }

                            if submitted && !state.nickname.is_empty() {
                                config.nickname = Some(state.nickname.clone());
                            }
                        });
                    }
                });
            });
        });
    });
}
