use egui::{RichText, Ui};
use runa_core::components::{
    ActiveCamera, AudioListener, AudioSource, Camera, CursorInteractable, MeshRenderer,
    PhysicsCollision, SpriteRenderer, Tilemap, Transform,
};
use runa_core::glam::{EulerRot, Quat, Vec3};
use runa_core::ocs::Object;

pub fn inspector_ui(ui: &mut Ui, object: &mut Object) {
    ui.label("Name");
    ui.text_edit_singleline(&mut object.name);
    ui.separator();

    if let Some(transform) = object.get_component_mut::<Transform>() {
        egui::CollapsingHeader::new("Transform")
            .default_open(true)
            .show(ui, |ui| {
                vec3_editor(ui, "Position", &mut transform.position);
                quat_editor(ui, transform);
                vec3_editor(ui, "Scale", &mut transform.scale);
            });
    }

    if let Some(camera) = object.get_component_mut::<Camera>() {
        egui::CollapsingHeader::new("Camera")
            .default_open(true)
            .show(ui, |ui| {
                vec3_editor(ui, "Position", &mut camera.position);
                vec3_editor(ui, "Target", &mut camera.target);
                ui.horizontal(|ui| {
                    ui.label("FOV");
                    let mut degrees = camera.fov.to_degrees();
                    if ui
                        .add(egui::DragValue::new(&mut degrees).speed(0.25))
                        .changed()
                    {
                        camera.fov = degrees.to_radians();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Near");
                    ui.add(egui::DragValue::new(&mut camera.near).speed(0.01));
                    ui.label("Far");
                    ui.add(egui::DragValue::new(&mut camera.far).speed(1.0));
                });
            });
    }

    if object.get_component::<ActiveCamera>().is_some() {
        component_badge(ui, "ActiveCamera", "Selected runtime camera");
    }

    if let Some(mesh_renderer) = object.get_component_mut::<MeshRenderer>() {
        egui::CollapsingHeader::new("MeshRenderer")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Vertices: {}", mesh_renderer.mesh.vertices.len()));
                ui.label(format!("Indices: {}", mesh_renderer.mesh.indices.len()));
                color_editor(ui, "Tint", &mut mesh_renderer.color);
            });
    }

    if let Some(sprite) = object.get_component::<SpriteRenderer>() {
        egui::CollapsingHeader::new("SpriteRenderer")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(if sprite.texture.is_some() {
                    "Texture: assigned"
                } else {
                    "Texture: none"
                });
            });
    }

    if let Some(tilemap) = object.get_component::<Tilemap>() {
        egui::CollapsingHeader::new("Tilemap")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Grid: {} x {}", tilemap.width, tilemap.height));
                ui.label(format!("Layers: {}", tilemap.layers.len()));
            });
    }

    if let Some(audio) = object.get_component::<AudioSource>() {
        egui::CollapsingHeader::new("AudioSource")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Volume: {:.2}", audio.volume));
                ui.label(format!("Looped: {}", audio.looped));
                ui.label(format!("Spatial: {}", audio.spatial));
            });
    }

    if let Some(listener) = object.get_component::<AudioListener>() {
        egui::CollapsingHeader::new("AudioListener")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Active: {}", listener.active));
                ui.label(format!("Volume: {:.2}", listener.volume));
                ui.label(format!(
                    "Stereo Separation: {:.2}",
                    listener.stereo_separation
                ));
            });
    }

    if let Some(interactable) = object.get_component::<CursorInteractable>() {
        egui::CollapsingHeader::new("CursorInteractable")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!(
                    "Bounds: {:.2}, {:.2}, {:.2}",
                    interactable.bounds_size.x,
                    interactable.bounds_size.y,
                    interactable.bounds_size.z
                ));
                ui.label(format!("Hovered: {}", interactable.is_hovered));
                ui.label(format!("Pressed: {}", interactable.is_pressed));
            });
    }

    if let Some(collision) = object.get_component::<PhysicsCollision>() {
        egui::CollapsingHeader::new("PhysicsCollision")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!(
                    "Size: {:.2}, {:.2}",
                    collision.size.x, collision.size.y
                ));
                ui.label(format!("Enabled: {}", collision.enabled));
            });
    }
}

fn vec3_editor(ui: &mut Ui, label: &str, value: &mut Vec3) {
    ui.label(label);
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut value.x).speed(0.05).prefix("x "));
        ui.add(egui::DragValue::new(&mut value.y).speed(0.05).prefix("y "));
        ui.add(egui::DragValue::new(&mut value.z).speed(0.05).prefix("z "));
    });
}

fn quat_editor(ui: &mut Ui, transform: &mut Transform) {
    let (mut x, mut y, mut z) = transform.rotation.to_euler(EulerRot::XYZ);
    x = x.to_degrees();
    y = y.to_degrees();
    z = z.to_degrees();

    ui.label("Rotation");
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui
            .add(egui::DragValue::new(&mut x).speed(0.5).prefix("x "))
            .changed();
        changed |= ui
            .add(egui::DragValue::new(&mut y).speed(0.5).prefix("y "))
            .changed();
        changed |= ui
            .add(egui::DragValue::new(&mut z).speed(0.5).prefix("z "))
            .changed();
    });

    if changed {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            x.to_radians(),
            y.to_radians(),
            z.to_radians(),
        );
    }
}

fn color_editor(ui: &mut Ui, label: &str, color: &mut [f32; 4]) {
    ui.label(label);
    ui.horizontal(|ui| {
        for channel in color.iter_mut() {
            ui.add(egui::DragValue::new(channel).range(0.0..=1.0).speed(0.01));
        }
    });
}

fn component_badge(ui: &mut Ui, label: &str, description: &str) {
    ui.group(|ui| {
        ui.label(RichText::new(label).strong());
        ui.label(description);
    });
}
