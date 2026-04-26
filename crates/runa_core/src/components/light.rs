use glam::Vec3;

use super::{Component, SerializedField, SerializedFieldAccess, SerializedFieldValue};

#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: Vec3::new(-0.3, -1.0, -0.4),
            color: Vec3::ONE,
            intensity: 1.0,
        }
    }
}

impl SerializedFieldAccess for DirectionalLight {
    fn serialized_fields(&self) -> Vec<SerializedField> {
        vec![
            SerializedField {
                name: "direction".to_string(),
                value: SerializedFieldValue::Vec3(self.direction.to_array()),
            },
            SerializedField {
                name: "color".to_string(),
                value: SerializedFieldValue::Vec3(self.color.to_array()),
            },
            SerializedField {
                name: "intensity".to_string(),
                value: SerializedFieldValue::F32(self.intensity),
            },
        ]
    }

    fn set_serialized_field(&mut self, field_name: &str, value: SerializedFieldValue) -> bool {
        match (field_name, value) {
            ("direction", SerializedFieldValue::Vec3(value)) => {
                self.direction = Vec3::from_array(value);
                true
            }
            ("color", SerializedFieldValue::Vec3(value)) => {
                self.color = Vec3::from_array(value);
                true
            }
            ("intensity", SerializedFieldValue::F32(value)) => {
                self.intensity = value;
                true
            }
            _ => false,
        }
    }
}

impl Component for DirectionalLight {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub color: Vec3,
    pub intensity: f32,
    pub radius: f32,
    pub falloff: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            color: Vec3::ONE,
            intensity: 4.0,
            radius: 6.0,
            falloff: 1.0,
        }
    }
}

impl SerializedFieldAccess for PointLight {
    fn serialized_fields(&self) -> Vec<SerializedField> {
        vec![
            SerializedField {
                name: "color".to_string(),
                value: SerializedFieldValue::Vec3(self.color.to_array()),
            },
            SerializedField {
                name: "intensity".to_string(),
                value: SerializedFieldValue::F32(self.intensity),
            },
            SerializedField {
                name: "radius".to_string(),
                value: SerializedFieldValue::F32(self.radius),
            },
            SerializedField {
                name: "falloff".to_string(),
                value: SerializedFieldValue::F32(self.falloff),
            },
        ]
    }

    fn set_serialized_field(&mut self, field_name: &str, value: SerializedFieldValue) -> bool {
        match (field_name, value) {
            ("color", SerializedFieldValue::Vec3(value)) => {
                self.color = Vec3::from_array(value);
                true
            }
            ("intensity", SerializedFieldValue::F32(value)) => {
                self.intensity = value;
                true
            }
            ("radius", SerializedFieldValue::F32(value)) => {
                self.radius = value.max(0.0);
                true
            }
            ("falloff", SerializedFieldValue::F32(value)) => {
                self.falloff = value.max(0.0);
                true
            }
            _ => false,
        }
    }
}

impl Component for PointLight {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
