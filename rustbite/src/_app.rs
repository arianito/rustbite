use glutin;
use gl;
use std;

pub struct app<'a> {
    pub init: Box<Fn() + 'a>,
    pub create: Box<Fn() + 'a>,
    pub update: Box<Fn() + 'a>,
}

impl<'a> app<'a> {
    pub fn run(&mut self) {
        
        (self.init)();

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("Rustbite")
            .with_vsync()
            .build(&events_loop)
            .unwrap();

        let _ = unsafe { window.make_current() };


        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.1, 0.15, 0.2, 1.0);
        }


        (self.create)();
        

        events_loop.run_forever(|event| {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            window.swap_buffers().unwrap();

            (self.update)();

            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } =>
                    events_loop.interrupt(),
                _ => ()
            }
        });


    }
}
