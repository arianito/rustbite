#[macro_use]
extern crate glium;
extern crate math;
extern crate pipeline;



use math::vec3;
use math::mat4;
use math::quat;

fn main() {

    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;

    let mut model = mat4::create_trs(&vec3::new(0.0,0.0,0.0), &quat::identify(), &vec3::new(0.5,0.5,0.5));

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(600, 400)
        .with_multisampling(4)
        .with_title("rust-game")
        .build_glium()
        .unwrap();


    let mut view = mat4::identify(1.0);
    let mut projection = mat4::ortho_window(2.0, 6.0/4.0, 200.0, -0.1);


    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }
    implement_vertex!(Vertex, position);


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


    let vertex1 = Vertex { position: [-1.0, -1.0, 0.0] };
    let vertex2 = Vertex { position: [1.0, 1.0, 0.0] };
    let vertex3 = Vertex { position: [-1.0, 1.0, 0.0] };
    
    let vertex4 = Vertex { position: [-1.0, -1.0, 0.0] };
    let vertex5 = Vertex { position: [1.0, -1.0, 0.0] };
    let vertex6 = Vertex { position: [-1.0, 1.0, 0.0] };

    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t: f32 = 0.0f32;
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.04, 0.09, 0.2, 1.0), 1.0);

        model = mat4::create_trs(&vec3::new(0.0,0.0,0.0), &quat::from_angle_axis((my/mx).atan() * math::Rad2Deg, &vec3::forward()), &vec3::one());

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

        t = t + 0.01;
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(w, h) => {
                    projection = mat4::ortho_window(2.0, w as f32 / h as f32, 200.0, -0.1);
                },
                glium::glutin::Event::MouseMoved(x, y) => {
                    mx = x as f32;
                    my = y as f32;
                },
                _ => (),
            }
        }
    }
}
