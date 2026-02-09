use eframe::egui::ViewportBuilder;
use eframe::{App, egui};
use egui::containers::menu;
use egui::{CentralPanel, IconData, SidePanel, TopBottomPanel};
use runa_core::World;

struct Editor {
    world: World,
    selected_object_id: Option<usize>,
    show_content_browser: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            world: World::default(),
            selected_object_id: None,
            show_content_browser: false,
        }
    }
}

fn load_icon() -> Option<IconData> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_bytes).ok()?;
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();

    Some(IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

impl App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
                // File
                ui.menu_button("File", |ui| {
                    if ui.button("New World").clicked() {
                        // TODO
                    }
                    if ui.button("Open...").clicked() {
                        // TODO
                    }
                    if ui.button("Save").clicked() {
                        // TODO
                    }
                    if ui.button("Save As...").clicked() {
                        // TODO
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Edit
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() { /* ... */ }
                    if ui.button("Redo").clicked() { /* ... */ }
                });

                // View
                ui.menu_button("View", |ui| {
                    if ui.button("Content Browser (Ctrl+Space)").clicked() {
                        self.show_content_browser = !self.show_content_browser;
                    }
                });

                // Window, Help и т.д. — по желанию
            });
        });

        SidePanel::left("hierarchy_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Hierarchy");
                ui.separator();
                ui.label("Empty World");
            });

        SidePanel::right("inspector_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Inspector");
                ui.separator();
                ui.label("No object selected");
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Viewport (Preview)");
            });
        });
        if self.show_content_browser {
            egui::Area::new(egui::Id::new("content_browser"))
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .interactable(true)
                .show(ctx, |ui| {
                    ui.set_min_size(egui::vec2(600.0, 400.0));
                    ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

                    ui.heading("Content Browser");
                    // ui.text_edit_singleline(&mut self.search_query);

                    // Список скриптов/ассетов
                    // for script in &self.registered_scripts {
                    //     if ui.button(script).clicked() {
                    //         // drag-and-drop логика
                    //     }
                    // }

                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_content_browser = false;
                    }
                });
        }
    }
}

fn main() -> eframe::Result {
    let icon_data = load_icon();

    let viewport = ViewportBuilder::default()
        .with_title("Runa Editor")
        .with_inner_size([1280.0, 720.0])
        .with_fullscreen(false);

    let viewport = if let Some(icon) = icon_data {
        viewport.with_icon(icon)
    } else {
        viewport
    };

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Runa Editor",
        options,
        Box::new(|cc| {
            setup_style(&cc.egui_ctx);
            Ok(Box::new(Editor::default()))
        }),
    )
}

fn setup_style(ctx: &egui::Context) {
    use egui::{Color32, Style, Visuals};

    let mut style = Style::default();

    // Тёмная тема в духе UE5 / Zed
    style.visuals = Visuals::dark();
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(40, 40, 45);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 70);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(80, 80, 90);
    style.visuals.panel_fill = Color32::from_rgb(10, 10, 10);
    style.visuals.window_fill = Color32::from_rgb(25, 25, 30);
    style.visuals.code_bg_color = Color32::from_rgb(255, 255, 255);
    style.visuals.hyperlink_color = Color32::from_rgb(100, 180, 255);
    style.visuals.selection.bg_fill = Color32::from_rgb(60, 100, 160);

    // Опционально: настроить шрифт
    style
        .text_styles
        .get_mut(&egui::TextStyle::Body)
        .unwrap()
        .size = 18.0;

    ctx.set_style(style);
}
