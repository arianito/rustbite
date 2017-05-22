#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


pub const PI: f32 = 3.141592653589793_f32;
pub const HALF_PI: f32 = 1.570796326794896_f32;
pub const RAD2_DEG: f32 = 57.295779513082321_f32;
pub const DEG2_RAD: f32 = 0.017453292519943_f32;
pub const EPSILON: f32 = 0.000001_f32;


mod _vec3;
mod _quat;
mod _mat4;

pub use _vec3::vec3;
pub use _quat::quat;
pub use _mat4::mat4;


