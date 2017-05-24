#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


extern crate glutin;
extern crate gl;


pub mod _math;
mod _vec3;
mod _quat;
mod _mat4;
mod _app;
mod _context;
mod _shader;

pub use _math as math;
pub use _vec3::vec3;
pub use _quat::quat;
pub use _mat4::mat4;
pub use _app::app;
pub use _context::context;
pub use _shader::shader;