use runa_engine::runa_core::ocs::{Script, ScriptContext};
use runa_engine::{Engine, RunaComponent, RunaScript, SerializedFieldAccess};

#[derive(RunaComponent, Default)]
struct TestComponent {
    pub hidden_public: f32,
    #[serialize_field]
    explicit_value: f32,
}

#[derive(RunaScript, Default)]
struct TestScript {
    pub hidden_public: f32,
    #[serialize_field]
    explicit_value: f32,
}

impl Script for TestScript {
    fn update(&mut self, _ctx: &mut ScriptContext, _dt: f32) {}
}

#[derive(RunaScript)]
#[runa(factory = "Self::new()")]
struct FactoryScript {
    #[serialize_field]
    explicit_value: f32,
}

impl FactoryScript {
    fn new() -> Self {
        Self {
            explicit_value: 42.0,
        }
    }
}

impl Script for FactoryScript {
    fn update(&mut self, _ctx: &mut ScriptContext, _dt: f32) {}
}

#[test]
fn derive_exposes_only_explicit_serialize_fields() {
    let component_fields = TestComponent::default().serialized_fields();
    let script_fields = TestScript::default().serialized_fields();

    assert_eq!(component_fields.len(), 1);
    assert_eq!(script_fields.len(), 1);
    assert_eq!(component_fields[0].name, "explicit_value");
    assert_eq!(script_fields[0].name, "explicit_value");
}

#[test]
fn derive_factory_attribute_uses_explicit_constructor_for_registration() {
    let mut engine = Engine::new();
    let metadata = engine.register::<FactoryScript>();
    let mut object = runa_engine::runa_core::ocs::Object::new("Factory");

    assert!(engine
        .runtime_registry()
        .add_type_to_object(&mut object, metadata.type_id()));
    let fields = object
        .get_component::<FactoryScript>()
        .unwrap()
        .serialized_fields();

    assert_eq!(fields.len(), 1);
    match fields[0].value {
        runa_engine::runa_core::components::SerializedFieldValue::F32(value) => {
            assert!((value - 42.0).abs() < f32::EPSILON);
        }
        ref other => panic!("unexpected field value: {other:?}"),
    }
}
