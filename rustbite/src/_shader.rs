use gl;
use std::ptr;

pub struct shader {
    vertex_shader: &'static [u8],
    fragment_shader: &'static [u8]
}

impl shader{
    pub fn new(v: &'static [u8], f: &'static [u8]) -> shader {
        shader {
            vertex_shader: v,
            fragment_shader: f,
        }
    }

    pub fn compile(&self) {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [self.vertex_shader.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [self.fragment_shader.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(fs);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
        }
    }
}