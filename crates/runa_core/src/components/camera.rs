use glam::{Mat4, Vec2, Vec3};

/// Camera projection type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectionType {
    /// Orthographic projection for 2D rendering.
    Orthographic,
    /// Perspective projection for 3D rendering.
    Perspective,
}

/// A shared camera component with 2D and 3D support.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Camera position in world space.
    pub position: Vec3,
    /// Look target for 3D cameras.
    pub target: Vec3,
    /// Up direction for 3D cameras.
    pub up: Vec3,

    /// Projection mode.
    pub projection: ProjectionType,

    // Orthographic projection parameters (2D)
    /// Orthographic camera size as width and height.
    pub ortho_size: Vec2,
    /// Near clipping plane.
    pub near: f32,
    /// Far clipping plane.
    pub far: f32,

    // Perspective projection parameters (3D)
    /// Field of view in radians for 3D cameras.
    pub fov: f32,

    /// Render viewport size.
    pub viewport_size: (u32, u32),
}

impl Camera {
    /// Creates a new orthographic camera for 2D rendering.
    ///
    /// # Arguments
    /// * `width` - Visible width
    /// * `height` - Visible height
    /// * `viewport_size` - Render window size
    pub fn new_ortho(width: f32, height: f32, viewport_size: (u32, u32)) -> Self {
        Self {
            position: Vec3::ZERO,
            target: Vec3::Z,
            up: Vec3::Y,
            projection: ProjectionType::Orthographic,
            ortho_size: Vec2::new(width / 10.0, height / 10.0),
            near: -1000.0,
            far: 1000.0,
            fov: 0.0, // Unused for orthographic projection
            viewport_size,
        }
    }

    /// Creates a new perspective camera for 3D rendering.
    ///
    /// # Arguments
    /// * `position` - Camera position
    /// * `target` - Look target
    /// * `up` - Up direction
    /// * `fov` - Field of view in radians
    /// * `near` - Near clipping plane
    /// * `far` - Far clipping plane
    /// * `viewport_size` - Render window size
    pub fn new_perspective(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        near: f32,
        far: f32,
        viewport_size: (u32, u32),
    ) -> Self {
        Self {
            position,
            target,
            up,
            projection: ProjectionType::Perspective,
            ortho_size: Vec2::ZERO, // Unused for perspective projection
            near,
            far,
            fov,
            viewport_size,
        }
    }

    /// Returns the view-projection matrix.
    pub fn matrix(&self) -> Mat4 {
        match self.projection {
            ProjectionType::Orthographic => self.ortho_matrix(),
            ProjectionType::Perspective => self.perspective_matrix(),
        }
    }

    /// Returns the orthographic projection matrix.
    fn ortho_matrix(&self) -> Mat4 {
        let half_width = self.ortho_size.x * 0.5;
        let half_height = self.ortho_size.y * 0.5;

        // orthographic_rh_gl uses Z in the -1..1 NDC range
        let proj = Mat4::orthographic_rh_gl(
            -half_width,
            half_width,
            -half_height,
            half_height,
            self.near,
            self.far,
        );

        let view = Mat4::from_translation(-self.position);

        proj * view
    }

    /// Returns the perspective projection matrix.
    fn perspective_matrix(&self) -> Mat4 {
        let aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let proj = Mat4::perspective_rh(self.fov, aspect, self.near, self.far);
        let view = Mat4::look_at_rh(self.position, self.target, self.up);
        proj * view
    }

    /// Sets the camera position.
    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    /// Sets the orthographic camera size.
    pub fn set_ortho_size(&mut self, size: Vec2) {
        self.ortho_size = size;
    }

    /// Sets the field of view for a perspective camera.
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    /// Returns the aspect ratio.
    pub fn aspect(&self) -> f32 {
        self.viewport_size.0 as f32 / self.viewport_size.1 as f32
    }

    /// Updates the viewport size.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.viewport_size = (width.max(1), height.max(1));
    }

    /// Converts screen coordinates to world coordinates for orthographic cameras.
    pub fn screen_to_world(&self, screen_pos: (f32, f32)) -> Vec2 {
        let (screen_x, screen_y) = screen_pos;
        let (viewport_width, viewport_height) = self.viewport_size;

        // Normalize to NDC
        let ndc_x = (screen_x / viewport_width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_y / viewport_height as f32) * 2.0;

        // Orthographic camera path
        let half_width = self.ortho_size.x * 0.5;
        let half_height = self.ortho_size.y * 0.5;

        // Match the aspect correction used in renderer.rs
        // aspect = (virtual_size.x / virtual_size.y) / (window_width / window_height)
        let virtual_aspect = self.ortho_size.x / self.ortho_size.y;
        let window_aspect = viewport_width as f32 / viewport_height as f32;
        let aspect_correction = virtual_aspect / window_aspect;

        let corrected_ndc_x = ndc_x / aspect_correction;

        let world_x = corrected_ndc_x * half_width + self.position.x;
        let world_y = ndc_y * half_height + self.position.y;

        Vec2::new(world_x, world_y)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new_perspective(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::Y,
            75.0_f32.to_radians(),
            0.1,
            1000.0,
            (1280, 720),
        )
    }
}
