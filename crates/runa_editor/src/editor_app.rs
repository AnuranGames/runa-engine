use std::sync::Arc;
use std::time::Instant;

use egui::{Color32, Layout, RichText};
use egui_wgpu::{Renderer as EguiRenderer, RendererOptions, ScreenDescriptor};
use egui_winit::State as EguiWinitState;
use runa_asset::load_window_icon;
use runa_core::components::{Mesh, MeshRenderer, PhysicsCollision, Transform};
use runa_core::glam::{Vec2, Vec3};
use runa_core::ocs::Object;
use runa_core::World;
use runa_render::{RenderTarget, Renderer};
use runa_render_api::RenderQueue;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::content_browser::ContentBrowserState;
use crate::editor_camera::EditorCameraController;
use crate::editor_settings::EditorSettings;
use crate::inspector::inspector_ui;

const INITIAL_VIEWPORT_SIZE: (u32, u32) = (960, 540);

pub fn run() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = EditorApp::new();
    event_loop.run_app(&mut app)
}

struct PanelState {
    hierarchy: bool,
    inspector: bool,
    content_browser: bool,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            hierarchy: true,
            inspector: true,
            content_browser: true,
        }
    }
}

pub struct EditorApp<'window> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'window>>,
    egui_state: Option<EguiWinitState>,
    egui_renderer: Option<EguiRenderer>,
    egui_ctx: egui::Context,

    world: World,
    scene_queue: RenderQueue,
    selection: Option<usize>,
    content_browser: ContentBrowserState,
    panels: PanelState,
    settings: EditorSettings,
    settings_open: bool,

    editor_camera: EditorCameraController,
    viewport_target: Option<RenderTarget>,
    viewport_texture_id: Option<egui::TextureId>,
    pending_viewport_size: (u32, u32),
    viewport_hovered: bool,
    modifiers: ModifiersState,

    status_line: String,
    last_frame_time: Instant,
}

impl<'window> EditorApp<'window> {
    fn new() -> Self {
        let project_root = std::env::current_dir().unwrap_or_default();
        Self {
            settings: EditorSettings::load(),
            settings_open: false,
            window: None,
            renderer: None,
            egui_state: None,
            egui_renderer: None,
            egui_ctx: egui::Context::default(),
            world: create_preview_world(),
            scene_queue: RenderQueue::new(),
            selection: Some(0),
            content_browser: ContentBrowserState::new(project_root),
            panels: PanelState::default(),
            editor_camera: EditorCameraController::new(),
            viewport_target: None,
            viewport_texture_id: None,
            pending_viewport_size: INITIAL_VIEWPORT_SIZE,
            viewport_hovered: false,
            modifiers: ModifiersState::default(),
            status_line:
                "Right mouse in viewport: look. WASD move, Space/Ctrl vertical, Shift boost."
                    .to_string(),
            last_frame_time: Instant::now(),
        }
    }

    fn ensure_viewport_target(&mut self) {
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        let Some(egui_renderer) = self.egui_renderer.as_mut() else {
            return;
        };

        let needs_recreate = self
            .viewport_target
            .as_ref()
            .map(|target| target.size() != self.pending_viewport_size)
            .unwrap_or(true);

        if !needs_recreate {
            return;
        }

        let target = renderer.create_render_target(self.pending_viewport_size);
        if let Some(texture_id) = self.viewport_texture_id {
            egui_renderer.update_egui_texture_from_wgpu_texture(
                renderer.device(),
                target.color_view(),
                wgpu::FilterMode::Linear,
                texture_id,
            );
        } else {
            let texture_id = egui_renderer.register_native_texture(
                renderer.device(),
                target.color_view(),
                wgpu::FilterMode::Linear,
            );
            self.viewport_texture_id = Some(texture_id);
        }
        self.viewport_target = Some(target);
    }

    fn update_scene_preview(&mut self) {
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        let Some(target) = self.viewport_target.as_ref() else {
            return;
        };

        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32().min(0.1);
        self.last_frame_time = now;

        self.editor_camera
            .set_viewport_hovered(self.viewport_hovered);
        self.editor_camera.update(dt);

        let camera = self.editor_camera.camera(target.size());
        let virtual_size = Vec2::new(target.size().0 as f32, target.size().1 as f32);

        self.scene_queue.clear();
        self.world.render(&mut self.scene_queue, 1.0);
        renderer.draw_to_target(target, &self.scene_queue, camera.matrix(), virtual_size);
    }

    fn draw_ui(&mut self) -> egui::FullOutput {
        let window = self.window.as_ref().unwrap();
        let egui_state = self.egui_state.as_mut().unwrap();
        let raw_input = egui_state.take_egui_input(window);
        let egui_ctx = self.egui_ctx.clone();
        egui_ctx.run(raw_input, |ctx| {
            self.build_ui(ctx);
        })
    }

    fn build_ui(&mut self, ctx: &egui::Context) {
        egui::Panel::top("editor_top_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.label("New Scene");
                    ui.label("Open Scene");
                    ui.label("Save Scene");
                });
                ui.menu_button("Editor", |ui| {
                    if ui.button("Settings").clicked() {
                        self.settings_open = true;
                        ui.close();
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.panels.hierarchy, "Hierarchy");
                    ui.checkbox(&mut self.panels.inspector, "Inspector");
                    ui.checkbox(&mut self.panels.content_browser, "Content Browser");
                });
                ui.separator();
                ui.add_enabled(false, egui::Button::new("Play In Window"));
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Runa Editor").strong());
                });
            });
        });

        egui::Panel::bottom("status_bar")
            .resizable(false)
            .exact_size(24.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.label(&self.status_line);
            });

        if self.panels.content_browser {
            egui::TopBottomPanel::bottom("content_browser")
                .resizable(true)
                .default_height(220.0)
                .min_height(120.0)
                .show(ctx, |ui| {
                    self.content_browser.ui(ui, &self.settings);
                    if let Some(message) = self.content_browser.take_message() {
                        self.status_line = message;
                    }
                });
        }

        if self.panels.hierarchy {
            egui::SidePanel::left("hierarchy_panel")
                .resizable(true)
                .default_width(240.0)
                .min_width(180.0)
                .show(ctx, |ui| {
                    ui.heading("Hierarchy");
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_salt("hierarchy_scroll")
                        .show(ui, |ui| {
                            for (index, object) in self.world.objects.iter().enumerate() {
                                let selected = self.selection == Some(index);
                                if ui
                                    .selectable_label(selected, object_title(index, object))
                                    .clicked()
                                {
                                    self.selection = Some(index);
                                }
                            }
                        });
                });
        }

        if self.panels.inspector {
            egui::SidePanel::right("inspector_panel")
                .resizable(true)
                .default_width(320.0)
                .min_width(220.0)
                .show(ctx, |ui| {
                    ui.heading("Inspector");
                    ui.separator();
                    if let Some(index) = self.selection {
                        if let Some(object) = self.world.objects.get_mut(index) {
                            inspector_ui(ui, object);
                        } else {
                            ui.label("Selection is out of bounds.");
                        }
                    } else {
                        ui.label("Select an object in the hierarchy.");
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Scene");
                ui.label("Editor camera preview");
            });
            ui.separator();

            let desired_size = ui.available_size().max(egui::vec2(64.0, 64.0));
            let pixels_per_point = ctx.pixels_per_point();
            self.pending_viewport_size = (
                (desired_size.x * pixels_per_point).round().max(1.0) as u32,
                (desired_size.y * pixels_per_point).round().max(1.0) as u32,
            );

            let frame = egui::Frame::canvas(ui.style())
                .fill(Color32::from_rgb(18, 20, 24))
                .inner_margin(egui::Margin::same(6));

            frame.show(ui, |ui| {
                if let Some(texture_id) = self.viewport_texture_id {
                    let response = ui.add(egui::Image::new((texture_id, desired_size)));
                    self.viewport_hovered = response.hovered();
                } else {
                    self.viewport_hovered = false;
                    ui.allocate_ui_with_layout(
                        desired_size,
                        Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.label("Viewport is initializing...");
                        },
                    );
                }
            });
        });

        self.settings_window(ctx);
    }

    fn render_frame(&mut self) {
        let full_output = self.draw_ui();
        self.ensure_viewport_target();
        self.update_scene_preview();

        let Some(window) = self.window.as_ref() else {
            return;
        };
        let Some(egui_state) = self.egui_state.as_mut() else {
            return;
        };
        egui_state.handle_platform_output(window, full_output.platform_output);

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        let Some(egui_renderer) = self.egui_renderer.as_mut() else {
            return;
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(renderer.device(), renderer.queue(), *id, image_delta);
        }

        let size = window.inner_size();
        let pixels_per_point = window.scale_factor() as f32;
        let paint_jobs = self
            .egui_ctx
            .tessellate(full_output.shapes, pixels_per_point);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width.max(1), size.height.max(1)],
            pixels_per_point,
        };

        let surface_texture = match renderer.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
            _ => return,
        };
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            renderer
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Editor UI Encoder"),
                });

        let mut command_buffers = egui_renderer.update_buffers(
            renderer.device(),
            renderer.queue(),
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Editor UI Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.07,
                            b: 0.09,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            let mut render_pass = render_pass.forget_lifetime();
            egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        command_buffers.push(encoder.finish());
        renderer.queue().submit(command_buffers);
        surface_texture.present();

        for id in &full_output.textures_delta.free {
            egui_renderer.free_texture(id);
        }
    }
}

impl<'window> ApplicationHandler for EditorApp<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        event_loop.set_control_flow(ControlFlow::Poll);

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Runa Editor")
                        .with_inner_size(egui_winit::winit::dpi::LogicalSize::new(1600.0, 960.0))
                        .with_min_inner_size(egui_winit::winit::dpi::LogicalSize::new(
                            1200.0, 720.0,
                        )),
                )
                .expect("Failed to create editor window"),
        );
        if let Ok(icon) = load_window_icon(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png"))
        {
            window.set_window_icon(Some(icon));
        }

        let renderer = Renderer::new(window.clone(), true);
        let egui_renderer = EguiRenderer::new(
            renderer.device(),
            renderer.surface_format(),
            RendererOptions::default(),
        );
        let egui_state = EguiWinitState::new(
            self.egui_ctx.clone(),
            egui::ViewportId::ROOT,
            window.as_ref(),
            Some(window.scale_factor() as f32),
            window.theme(),
            Some(renderer.device().limits().max_texture_dimension_2d as usize),
        );

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.egui_renderer = Some(egui_renderer);
        self.egui_state = Some(egui_state);
        setup_style(&self.egui_ctx);
        self.egui_ctx.set_zoom_factor(self.settings.ui_scale);
        self.content_browser.refresh(&self.settings);
        self.ensure_viewport_target();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        let camera_captured = self.editor_camera.handle_window_event(window, &event);
        if let Some(egui_state) = self.egui_state.as_mut() {
            let response = egui_state.on_window_event(window, &event);
            if response.repaint || camera_captured {
                window.request_redraw();
            }
        }

        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed()
                    && !event.repeat
                    && self.modifiers.control_key()
                    && matches!(event.physical_key, PhysicalKey::Code(KeyCode::Space))
                {
                    self.panels.content_browser = !self.panels.content_browser;
                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                self.editor_camera.shutdown(window);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize((size.width, size.height));
                }
            }
            WindowEvent::RedrawRequested => self.render_frame(),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.editor_camera.handle_device_event(&event);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

impl<'window> EditorApp<'window> {
    fn settings_window(&mut self, ctx: &egui::Context) {
        if !self.settings_open {
            return;
        }

        let mut open = self.settings_open;
        egui::Window::new("Editor Settings")
            .open(&mut open)
            .resizable(true)
            .default_width(460.0)
            .show(ctx, |ui| {
                ui.heading("External Editor");
                ui.horizontal(|ui| {
                    ui.label("Executable");
                    ui.text_edit_singleline(&mut self.settings.external_editor_executable);
                });
                ui.label("Arguments");
                ui.label("One argument per line. Use {file} as placeholder.");
                ui.add(
                    egui::TextEdit::multiline(&mut self.settings.external_editor_args)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );

                ui.separator();
                ui.heading("Interface");
                ui.horizontal(|ui| {
                    ui.label("UI Scale");
                    if ui
                        .add(egui::Slider::new(&mut self.settings.ui_scale, 0.75..=2.0))
                        .changed()
                    {
                        ctx.set_zoom_factor(self.settings.ui_scale);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Icon Size");
                    ui.add(
                        egui::Slider::new(&mut self.settings.content_icon_size, 32.0..=96.0)
                            .step_by(1.0),
                    );
                });
                if ui
                    .checkbox(&mut self.settings.show_hidden_files, "Show Hidden Files")
                    .changed()
                {
                    self.content_browser.refresh(&self.settings);
                }

                ui.separator();
                ui.heading("Actions");
                ui.horizontal(|ui| {
                    if ui.button("Use Zed").clicked() {
                        self.settings.external_editor_executable = "zed".to_string();
                        self.settings.external_editor_args = "{file}".to_string();
                    }
                    if ui.button("Use VS Code").clicked() {
                        self.settings.external_editor_executable = "code".to_string();
                        self.settings.external_editor_args = "--goto\n{file}".to_string();
                    }
                });

                ui.separator();
                if ui.button("Save Settings").clicked() {
                    match self.settings.save() {
                        Ok(()) => {
                            self.content_browser.refresh(&self.settings);
                            self.status_line = "Editor settings saved.".to_string();
                        }
                        Err(error) => {
                            self.status_line = format!("Failed to save settings: {error}");
                        }
                    }
                }
            });
        self.settings_open = open;
    }
}

fn create_preview_world() -> World {
    let mut world = World::default();

    let mut cube = Object::new();
    cube.name = "Preview Cube".to_string();
    let mut cube_transform = Transform::default();
    cube_transform.position = Vec3::new(0.0, 0.6, 0.0);
    cube_transform.scale = Vec3::splat(1.2);
    cube.add_component(cube_transform);
    let mut cube_mesh = MeshRenderer::new(Mesh::cube(1.5));
    cube_mesh.color = [1.0, 0.55, 0.2, 1.0];
    cube.add_component(cube_mesh);
    world.objects.push(cube);

    let mut floor = Object::new();
    floor.name = "Floor".to_string();
    let mut floor_transform = Transform::default();
    floor_transform.position = Vec3::new(0.0, -1.5, 0.0);
    floor_transform.scale = Vec3::new(8.0, 0.2, 8.0);
    floor.add_component(floor_transform);
    let mut floor_mesh = MeshRenderer::new(Mesh::cube(1.0));
    floor_mesh.color = [0.24, 0.27, 0.32, 1.0];
    floor.add_component(floor_mesh);
    floor.add_component(PhysicsCollision::new(8.0, 8.0));
    world.objects.push(floor);

    world
}

fn object_title(index: usize, object: &Object) -> String {
    if object.name.is_empty() {
        format!("Object {}", index)
    } else {
        object.name.clone()
    }
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.interaction.selectable_labels = false;
    ctx.set_style(style);
}
