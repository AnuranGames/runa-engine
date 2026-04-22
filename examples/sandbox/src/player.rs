use runa_asset::load_image;
use runa_engine::{Engine, RunaArchetype, RunaComponent, RunaScript};
use runa_core::{
    components::{ui::CanvasSpace, ActiveCamera, Camera, Canvas, Collider2D, SpriteRenderer, Transform},
    glam::Vec3,
    input_system::*,
    ocs::{Object, Script, ScriptContext, World},
};

#[derive(RunaComponent)]
pub struct Health {
    pub current: i32,
}

impl Health {
    pub fn new(current: i32) -> Self {
        Self { current }
    }
}


#[derive(RunaScript)]
pub struct PlayerController {
    speed: f32,
    direction: Vec3,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            speed: 16.0,
            direction: Vec3::ZERO,
        }
    }
}

impl Script for PlayerController {
    fn start(&mut self, ctx: &mut ScriptContext) {
        if let Some(transform) = ctx.get_component_mut::<Transform>() {
            transform.position = Vec3::new(0.0, 0.0, 0.0);
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }

        let _ = ctx.get_component::<Health>().map(|health| health.current);
    }

    fn update(&mut self, ctx: &mut ScriptContext, _dt: f32) {
        self.direction = Vec3::ZERO;

        if Input::is_key_pressed(KeyCode::KeyW) {
            self.direction.y = 1.0;
        }
        if Input::is_key_pressed(KeyCode::KeyS) {
            self.direction.y = -1.0;
        }
        if Input::is_key_pressed(KeyCode::KeyD) {
            self.direction.x = 1.0;
        }
        if Input::is_key_pressed(KeyCode::KeyA) {
            self.direction.x = -1.0;
        }

        let Some(current_position) = ctx
            .get_component::<Transform>()
            .map(|transform| transform.position)
        else {
            return;
        };

        println!("Player: {}", current_position); 

        let movement = self.direction.normalize_or_zero() * self.speed * _dt;
        let next_position = current_position + movement;

        if !ctx.would_collide_2d_at(next_position.truncate()) {
            if let Some(transform) = ctx.get_component_mut::<Transform>() {
                transform.position = next_position;
            }
        }
    }
}


#[derive(RunaScript)]
pub struct PlayerCameraFollow {
    #[serialize_field]
    lock_z: f32,
}

impl PlayerCameraFollow {
    pub fn new() -> Self {
        Self { lock_z: 0.0 }
    }
}

impl Script for PlayerCameraFollow {
    fn late_update(&mut self, ctx: &mut ScriptContext, _dt: f32) {
        let Some(player_id) = ctx.find_first_with::<PlayerController>() else {
            return;
        };
        let Some(player_position) = ctx
            .get_object(player_id)
            .and_then(|player| player.get_component::<Transform>())
            .map(|transform| transform.position)
        else {
            return;
        };
        let Some(transform) = ctx.get_component_mut::<Transform>() else {
            return;
        };

        // Hard follow keeps the player and camera on the same fixed-step path.
        // This avoids the visible screen-space jitter that appears when the
        // camera smooths toward a target while the target itself is interpolated.

        println!("Camera: {}", transform.position); 
        
        transform.position = Vec3::new(player_position.x, player_position.y, self.lock_z);
    }
}

pub fn create_player() -> Object {
    Object::new("Player")
        .with(SpriteRenderer::new(Some(load_image!("assets/art/Charactert.png"))))
        .with(Collider2D::new(2.0, 2.0))
        .with(Health::new(100))
        .with(PlayerController::new())
}

pub fn create_player_camera() -> Object {
    Object::new("Player Camera")
        .with(Camera::new_ortho(32.0, 18.0))
        .with(ActiveCamera)
        .with(Canvas::new(CanvasSpace::Camera))
        .with(PlayerCameraFollow::new())
}

#[derive(RunaArchetype)]
#[runa(name = "player")]
pub struct PlayerArchetype;

impl PlayerArchetype {
    pub fn create(world: &mut World) -> u64 {
        world.spawn(create_player())
    }
}

#[derive(RunaArchetype)]
#[runa(name = "player_camera")]
pub struct PlayerCameraArchetype;

impl PlayerCameraArchetype {
    pub fn create(world: &mut World) -> u64 {
        world.spawn(create_player_camera())
    }
}

pub fn register_types(engine: &mut Engine) {
    engine.register::<Health>();
    engine.register::<PlayerController>();
    engine.register::<PlayerCameraFollow>();
    engine.register_archetype::<PlayerArchetype>();
    engine.register_archetype::<PlayerCameraArchetype>();
}
