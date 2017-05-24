use gl;
use std;

pub struct shader {
    vertex_shader: &'static [u8],
    fragment_shader: &'static [u8],
    program: gl::types::GLuint
}

impl shader {

    pub fn new(v: &'static [u8], f: &'static [u8]) -> shader {
        shader {
            vertex_shader: v,
            fragment_shader: f,
            program: 0,
        }
    }

    pub fn compile(&mut self) {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [self.vertex_shader.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [self.fragment_shader.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            self.program = gl::CreateProgram();
            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
        }
    }

    pub fn use_here(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}