#![allow(non_camel_case_types)]
extern crate rustbite;

use rustbite::{vec3, mat4, quat, app, shader};


fn main() {


    let mut model: mat4;
    let view = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());
    let mut projection = mat4::ortho_window(2.0, 1.0, -0.1, 200.0);

    let mut sim = shader::new(b"
        #version 140

        in vec3 position;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    \0",b"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1,1,0,0.5);
        }
    \0");


    let init = Box::new(|| {

    });

    let create = Box::new(|| {
        sim.compile();
    });

    let update = Box::new(|| {
        
    });
    
    let mut x = app {
        init: init,
        create: create,
        update: update
    };
    x.run();

}



















/*
    let nw = time::now().tm_nsec;
    for i in 0..5000{
        let m = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());

        let mut m2= mat4::inverse(m);
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());
        m2= mat4::inverse(m2);
    }
    let ok = (time::now().tm_nsec - nw) as f32 /1000000.0;
    println!("{}ms", ok);

    return;


/*
    let mut model: mat4;
    let view = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());
    let mut projection = mat4::ortho_window(2.0, sw / sh, -0.1, 200.0);

    */


    
    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;

    let mut sw: f32 = 500.0;
    let mut sh: f32 = 500.0;


    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(sw as u32, sh as u32)
        .with_multisampling(4)
        .with_title("rust-game")
        .with_vsync()
        .build_glium()
        .unwrap();




    #[derive(Copy, Clone)]
    pub struct vert {
        position: [f32; 3],
    }

    implement_vertex!(vert, position);


    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1,1,0,0.5);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();


    let shape = vec![vert { position: [-1.0, -1.0, 0.0] },
                     vert { position: [1.0, 1.0, 0.0] },
                     vert { position: [-1.0, 1.0, 0.0] }];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut t: f32 = 0.0f32;
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((t%1.0, 0.09, 0.2, 1.0), 1.0);


        let rat = 2.0 * sh / sw;

        let mvc = mat4::inverse(view) * vec3::new(mx / sw * 2.0 - 2.0, -my / sh * rat + rat, 0.0);

        model = mat4::create_trs(mvc, quat::from_angle_axis(t, vec3::forward()), vec3::one());


        let uniforms = uniform! {
            model: model.source,
            view: view.source,
            projection: projection.source,
        };



        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        

        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();

            

        t = t + 0.1;
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(w, h) => {
                    sw = w as f32 / 2.0;
                    sh = h as f32 / 2.0;
                    println!("{}, {}", sw, sh);
                    projection = mat4::ortho_window(2.0, sw / sh, -0.1, 200.0);
                }
                glium::glutin::Event::MouseMoved(x, y) => {
                    mx = x as f32;
                    my = y as f32;
                }
                _ => (),
            }
        }
    }


    */