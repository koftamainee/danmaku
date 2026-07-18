use danmaku::{Danmaku, SharedDanmaku};
use renderer::{
    BeginParams, BlendMode, Camera, Color, FilterMode, GraphicsDevice, OrthographicCamera,
    RendererError, SpriteBatch,
};
use scripting::ScenarioRunner;
use std::sync::Arc;
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
    danmaku: SharedDanmaku,
    scenario: ScenarioRunner,
}

impl Game {
    fn update(&self) {
        self.scenario.update();
        self.danmaku.borrow_mut().update();
    }

    fn render(&mut self) {

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

                let camera = OrthographicCamera::default();

                let sprite_batch =
                    SpriteBatch::new(&graphics_device).expect("failed to create sprite batch");

                let danmaku = Danmaku::shared();

                let scenario = ScenarioRunner::new("base", danmaku.clone())
                    .expect("failed to create scenario runner");

                *self = App::Running(Game {
                    window,
                    danmaku,
                    graphics_device,
                    camera,
                    sprite_batch,
                    scenario,
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
