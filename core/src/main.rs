#[macro_use]
extern crate glium;
extern crate math;
extern crate pipeline;




use math::vec3;
use math::mat4;
use math::quat;

fn main() {

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().with_dimensions(400,400).with_multisampling(4).build_glium().unwrap();



    let mut model = mat4::identify(1.0);
    
    let mut view = mat4::identify(1.0);
    let mut projection = mat4::ortho(10.0, 10.0, -10.0, -10.0, 200.0, -0.1);


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
            color = vec4(1,1,0,1);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let vertex1 = Vertex { position: [-5.0, -5.0, 0.0] };
    let vertex2 = Vertex { position: [ -5.0,  0.0, 0.0] };
    let vertex3 = Vertex { position: [ 0.0, 0.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t : f32 = 0.0f32;
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.04, 0.09, 0.2, 1.0), 1.0);

        model = mat4::trs(&vec3::new(0.0, 3.0, 0.0), &quat::from_angle_axis(t, &vec3::forward()), &vec3::one());

        let uniforms = uniform! {
            model: model.source,
            view: view.source,
            projection: projection.source,
        };


        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();

        t = t + 0.01;
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
