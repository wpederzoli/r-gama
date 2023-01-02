use glium;
use glium::{glutin, Surface};

pub struct Game {
    event_loop: glutin::event_loop::EventLoop<()>,
    display: glium::Display,
}

impl Game {
    pub fn new(title: &String) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title(title);
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Self {
            event_loop,
            display,
        }
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }

            let mut target = self.display.draw();
            target.clear_color_and_depth((0.2, 0.3, 0.5, 1.0), 1.0);
            target.finish().unwrap();
        })
    }
}
