use glutin;
use gl;

pub struct app {
    pub init: Option<fn()>,
    pub update: Option<fn()>
}

impl app {
    pub fn new()-> app {
        return app {
            init: None,
            update: None
        };
    }

    pub fn run(&mut self) {
        
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("A fantastic window!")
            .build(&events_loop)
            .unwrap();

        let _ = unsafe { window.make_current() };


        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(1.0, 1.0, 0.0, 1.0);
        }


        events_loop.run_forever(|event| {
            
                
            unsafe {
                gl::Clear(gl::COLOR_BU  FFER_BIT);
            }

            window.swap_buffers().unwrap();





            match self.update {
                Some(x) => x(),
                None => return,
            }

            
            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } =>
                    events_loop.interrupt(),
                _ => ()
            }
        });

        match self.init {
            Some(x) => x(),
            None => return,
        }

    }
}
