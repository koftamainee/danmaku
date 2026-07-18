use std::cell::RefCell;
use std::rc::Rc;

use content::Content;
use engine::bullet::BulletSystem;
use engine::rng::Rng;
use renderer::{BeginParams, BlendMode, Color, FilterMode, GraphicsDevice, OrthographicCamera, RendererError, SpriteBatch};

pub struct Danmaku {
    pub graphics_device: GraphicsDevice,
    pub camera: OrthographicCamera,
    pub sprite_batch: SpriteBatch,

    pub bullet_system: BulletSystem,
    pub rng: Rng,
    pub content: Content,
    pub frame: u32,
}

pub type SharedDanmaku = Rc<RefCell<Danmaku>>;

impl Danmaku {
    pub fn new() -> Self {
        Self {
            bullet_system: BulletSystem::new(1000, glam::Vec2::new(320.0, 240.0)),
            rng: Rng::new(42),
            content: Content::new(),
            frame: 0,
        }
    }

    pub fn update(&mut self) {
        self.bullet_system.update();
        self.frame += 1;
    }
    
    pub fn render(&mut self) {
        let mut frame = match self.graphics_device.begin_frame() {
            Ok(frame) => frame,
            Err(RendererError::SurfaceOutdated) => {
                let size = self.window.inner_size();
                self.graphics_device.resize(size.width, size.height);
                return;
            }
            Err(err) => {
                eprintln!("render error: {err}");
                return;
            }
        };

        let (w, h) = self.graphics_device.surface_size();
        self.camera.zoom = (w as f32 / WORLD_W).min(h as f32 / WORLD_H);
        let vp = self.camera.view_projection(w as f32, h as f32);

        frame.clear(Color::CORNFLOWER_BLUE);

        self.sprite_batch.begin(
            &self.graphics_device,
            BeginParams {
                view_projection: vp,
                blend_mode: BlendMode::AlphaBlend,
                viewport: None,
                sampler: FilterMode::Nearest,
            },
        );

        self.sprite_batch
            .end(&self.graphics_device, &mut frame)
            .unwrap();

        self.graphics_device.end_frame(frame);
    }

    pub fn shared() -> SharedDanmaku {
        Rc::new(RefCell::new(Danmaku::new()))
    }
}

