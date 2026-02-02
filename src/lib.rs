use std::sync::Arc;

use winit::{
    application::ApplicationHandler, 
    event::*, event_loop::{ActiveEventLoop, EventLoop}, 
    window::Window,
    keyboard::{KeyCode, PhysicalKey}
};


pub struct State {
   // surface: wgpu::Surface<'static>,
    window: Arc<Window>
}

pub struct App {
    state: Option<State>
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None
        }
    }

}

impl ApplicationHandler<State> for App  {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        
        println!("Creating new window..");
        
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).expect("Não foi possivel criar janela em 'resumed'"));

        self.state = Some(pollster::block_on(State::new(window)).expect("Não foi possivel bloquear em 'resumed'"));
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.render();
            },
            WindowEvent::KeyboardInput {
                event: 
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            }
            _ => {}
        }
    }

}


impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok( Self { window })
    }


    pub fn resize(&mut self, _width: u32, _height: u32) {
        //....
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        //....
    }
}


pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
