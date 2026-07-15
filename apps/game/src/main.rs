use std::sync::Arc;
use content::Content;
use renderer::{
    BlendMode, Camera, Color, FilterMode, GraphicsDevice,
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

struct Game {
    window: Arc<Window>,
    graphics_device: GraphicsDevice,
    content: Content,
    camera: OrthographicCamera,
    sprite_batch: SpriteBatch,
    tick: u32,
}

impl Game {
    fn update(&mut self) {
        self.tick = self.tick.wrapping_add(1);
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

                let content = Content::new();
                let camera = OrthographicCamera::default();

                let sprite_batch =
                    SpriteBatch::new(&graphics_device).expect("failed to create sprite batch");

                *self = App::Running(Game {
                    window,
                    graphics_device,
                    content,
                    camera,
                    sprite_batch,
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
