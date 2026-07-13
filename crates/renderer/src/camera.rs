use glam::{Mat4, Vec2};

pub trait Camera {
    fn view_projection(&self, screen_width: f32, screen_height: f32) -> Mat4;
}

pub struct OrthographicCamera {
    pub position: Vec2,
    pub zoom: f32,
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Camera for OrthographicCamera {
    fn view_projection(&self, screen_width: f32, screen_height: f32) -> Mat4 {
        let half_w = screen_width * 0.5 / self.zoom;
        let half_h = screen_height * 0.5 / self.zoom;

        glam::camera::rh::proj::directx::orthographic(
            -half_w + self.position.x,
             half_w + self.position.x,
            -half_h + self.position.y,
             half_h + self.position.y,
            -1.0,
             1.0,
        )
    }
}
