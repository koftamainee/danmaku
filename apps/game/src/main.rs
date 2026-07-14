use std::collections::HashMap;
use std::sync::Arc;
use content::{AssetPath, Content, Handle};
use core::bullet::{Bullet, BulletKey, BulletKind};
use glam::Vec2;
use renderer::{
    Atlas, BlendMode, Camera, Color, DrawParams, FilterMode, GraphicsDevice,
    OrthographicCamera, RendererError, SpriteBatch, BeginParams,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

const WORLD_W: f32 = 640.0;
const WORLD_H: f32 = 480.0;

struct VisualData {
    sprite_name: String,
}

struct Game {
    window: Arc<Window>,
    graphics_device: GraphicsDevice,
    content: Content,
    camera: OrthographicCamera,
    sprite_batch: SpriteBatch,

    bullet_system: core::bullet::BulletSystem,
    bullet_atlas: Handle<Atlas>,
    visuals: HashMap<BulletKey, VisualData>,

    tick: u32,
}

impl Game {
    fn spawn_bullet(
        &mut self,
        sprite_name: &str,
        position: Vec2,
        angle: f32,
        speed: f32,
        angular_vel: f32,
        angular_accel: f32,
    ) {
        let key = self.bullet_system.spawn(Bullet {
            position,
            lifetime: 500,
            kind: BulletKind::Root {
                speed,
                acceleration: 0.0,
                min_speed: 0.0,
                max_speed: 0.0,
                angle,
                angular_velocity: angular_vel,
                angular_acceleration: angular_accel,
                min_angular_velocity: 0.0,
                max_angular_velocity: 0.0,
            },
        });

        self.visuals.insert(
            key,
            VisualData {
                sprite_name: sprite_name.to_string(),
            },
        );
    }

    fn update(&mut self) {
        self.bullet_system.update();
        self.tick = self.tick.wrapping_add(1);

            let angle_offset = self.tick as f32 * 0.12;
            for arm in 0..6 {
                let a = angle_offset + arm as f32 * std::f32::consts::PI / 3.0;
                self.spawn_bullet("bullet_yellow", Vec2::ZERO, a, 2.0, 0.03, 0.0);
                self.spawn_bullet("bullet_orange", Vec2::ZERO, a, 2.5, 0.03, 0.0);
                self.spawn_bullet("bullet_red", Vec2::ZERO, a, 3.0, 0.03, -0.0003);
            }
    }

    fn render(&mut self) {
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

        let atlas = self.content.get(self.bullet_atlas).unwrap();

        for key in self.bullet_system.render_order() {
            if let Some(bullet) = self.bullet_system.get(*key) {
                if let Some(vis) = self.visuals.get(key) {
                    if let Some(region) = atlas.get(&vis.sprite_name) {
                        self.sprite_batch.draw(
                            &self.graphics_device,
                            DrawParams {
                                texture: atlas.texture(),
                                source: Some(region.src()),
                                position: bullet.position,
                                scale: Vec2::splat(3.0),
                                color: Color::WHITE,
                                ..DrawParams::new(atlas.texture())
                            },
                        );
                    }
                }
            }
        }

        self.sprite_batch
            .end(&self.graphics_device, &mut frame)
            .unwrap();

        self.graphics_device.end_frame(frame);
    }
}

#[derive(Default)]
enum App {
    #[default]
    Uninitialized,
    Running(Game),
}

impl App {
    fn running_mut(&mut self) -> Option<&mut Game> {
        match self {
            App::Uninitialized => None,
            App::Running(game) => Some(game),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            App::Running(_) => return,
            App::Uninitialized => {
                let window_attributes = Window::default_attributes().with_title("danmaku");
                let window = event_loop
                    .create_window(window_attributes)
                    .expect("failed to create window");

                let window = Arc::new(window);
                let size = window.inner_size();

                let graphics_device = pollster::block_on(GraphicsDevice::new(
                    window.clone(),
                    size.width,
                    size.height,
                    false,
                ))
                .expect("failed to create renderer");

                let mut content = Content::new();

                let atlas_path =
                    AssetPath::parse("base:bullets/EoSD_bullets.json").expect("invalid path");
                let bullet_atlas = content
                    .load::<Atlas>(&atlas_path, (&graphics_device).into())
                    .expect("failed to load atlas");

                let camera = OrthographicCamera::default();

                let sprite_batch =
                    SpriteBatch::new(&graphics_device).expect("failed to create sprite batch");

                let bullet_system =
                    core::bullet::BulletSystem::new(30000, Vec2::new(WORLD_W, WORLD_H));

                *self = App::Running(Game {
                    window,
                    graphics_device,
                    content,
                    camera,
                    sprite_batch,
                    bullet_system,
                    bullet_atlas,
                    visuals: HashMap::new(),
                    tick: 0,
                });
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(game) = self.running_mut() else {
            return;
        };

        if game.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                game.graphics_device.resize(new_size.width, new_size.height);
            }
            WindowEvent::RedrawRequested => {
                game.update();
                game.render();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(game) = self.running_mut() else {
            return;
        };

        game.window.request_redraw();
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::Uninitialized;
    event_loop.run_app(&mut app).expect("event loop error");
}
