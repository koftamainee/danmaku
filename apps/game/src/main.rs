use std::sync::Arc;
use content::{AssetPath, Content, Handle};
use core::bullet::Bullet;
use glam::Vec2;
use renderer::{
    Atlas, BlendMode, Camera, Color, DrawParams, FilterMode, GraphicsDevice,
    OrthographicCamera, Rect, RendererError, SpriteBatch, BeginParams,
    AtlasLoadContext, Region,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

const WORLD_W: f32 = 640.0;
const WORLD_H: f32 = 480.0;

struct Game {
    window: Arc<Window>,
    graphics_device: GraphicsDevice,
    content: Content,
    camera: OrthographicCamera,
    sprite_batch: SpriteBatch,

    bullet_system: core::bullet::BulletSystem,
    bullet_atlas_key: slotmap::DefaultKey,

    tick: u32,
}

impl Game {
    fn update(&mut self) {
        self.bullet_system.update();
        self.tick = self.tick.wrapping_add(1);

        let bullet_yellow =
            core::sprite_handle::SpriteHandle::from_atlas(self.bullet_atlas_key, 9);
        let bullet_orange =
            core::sprite_handle::SpriteHandle::from_atlas(self.bullet_atlas_key, 10);
        let bullet_red =
            core::sprite_handle::SpriteHandle::from_atlas(self.bullet_atlas_key, 2);

        let angle_offset = self.tick as f32 * 0.12;
        for arm in 0..6 {
            let a = angle_offset + arm as f32 * std::f32::consts::PI / 3.0;
            self.bullet_system.spawn(
                Bullet::root(0.0, 0.0, bullet_yellow)
                    .speed(2.0)
                    .angle(a)
                    .angular_velocity(0.03)
                    .lifetime(500)
                    .build(),
            );
            self.bullet_system.spawn(
                Bullet::root(0.0, 0.0, bullet_orange)
                    .speed(2.5)
                    .angle(a)
                    .angular_velocity(0.03)
                    .lifetime(500)
                    .build(),
            );
            self.bullet_system.spawn(
                Bullet::root(0.0, 0.0, bullet_red)
                    .speed(3.0)
                    .angle(a)
                    .angular_velocity(0.03)
                    .angular_acceleration(-0.0003)
                    .lifetime(500)
                    .build(),
            );
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

        for instance in self.bullet_system.render_instances() {
            match instance.sprite {
                core::sprite_handle::SpriteHandle::Atlas { key, index } => {
                    let handle = Handle::<Atlas>::from_key(key);
                    if let Some(atlas) = self.content.get(handle) {
                        let region = &atlas.regions[index as usize];
                        self.sprite_batch.draw(
                            &self.graphics_device,
                            DrawParams {
                                texture: &atlas.gpu,
                                source: Some(region.src),
                                position: instance.position,
                                rotation: instance.rotation,
                                scale: Vec2::splat(3.0),
                                color: Color::WHITE,
                                ..DrawParams::new(&atlas.gpu)
                            },
                        );
                    }
                }
                core::sprite_handle::SpriteHandle::Texture { key: _ } => {}
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

                let bullet_atlas_key = {
                    let regions: Vec<Region> = vec![
                        Region { src: Rect::new(0.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(16.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(32.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(48.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(64.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(80.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(96.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(112.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(128.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(144.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(160.0, 48.0, 16.0, 16.0), hitbox: None },
                        Region { src: Rect::new(176.0, 48.0, 16.0, 16.0), hitbox: None },
                    ];
                    let handle = content
                        .load::<Atlas>(
                            &AssetPath::parse("base:bullets/EoSD_bullets.png").unwrap(),
                            AtlasLoadContext {
                                gpu: &graphics_device,
                                regions,
                                label: Some("Bullet atlas"),
                            },
                        )
                        .unwrap();
                    handle.key()
                };

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
                    bullet_atlas_key,
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
