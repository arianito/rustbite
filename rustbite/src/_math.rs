use std;

pub const PI: f32 = 3.141592653589793_f32;
pub const HALF_PI: f32 = 1.570796326794896_f32;
pub const RAD2_DEG: f32 = 57.295779513082321_f32;
pub const DEG2_RAD: f32 = 0.017453292519943_f32;
pub const EPSILON: f32 = 0.000001_f32;

pub fn inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5f3759df - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

pub fn sin(a: f32) -> f32 {
    return a.sin();
}

pub fn cos(a: f32) -> f32 {
    return a.cos();
}

pub fn acos(a: f32) -> f32 {
    return a.acos();
}

pub fn atan(a: f32) -> f32 {
    return a.atan();
}

pub fn sqrt(a: f32) -> f32 {
    return a.sqrt();
}
