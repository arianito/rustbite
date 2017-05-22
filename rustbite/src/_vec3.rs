use std;
use PI;

pub struct vec3 {
    pub source: [f32; 3]
}


impl std::ops::Index<usize> for vec3 {
     type Output = f32;

     fn index(&self, idx: usize) -> &f32 {
        match idx {
            1 => return &self.source[0],
            2 => return &self.source[1],
            3 => return &self.source[2],
            _ => return &self.source[0]
        }
    }
}


impl std::ops::IndexMut<usize> for vec3 {
     fn index_mut(&mut self, idx: usize) -> &mut f32 {
        match idx {
            1 => return &mut self.source[0],
            2 => return &mut self.source[1],
            3 => return &mut self.source[2],
            _ => return &mut self.source[0]
        }
    }
}

impl std::ops::Add for vec3{
    type Output = vec3;

    fn add(self, other: vec3) -> vec3 {
        vec3::add(&self, &other)
    }
}



impl std::ops::Div<f32> for vec3{
    type Output = vec3;

    fn div(self, other: f32) -> vec3 {
        vec3::div(&self, other)
    }
}


impl std::ops::Sub for vec3{
    type Output = vec3;

    fn sub(self, other: vec3) -> vec3 {
        vec3::sub(&self, &other)
    }
}


impl vec3 {

    // constants
    pub fn zero() -> vec3 {
        return vec3{
            source: [0.0f32,0.0f32,0.0f32]
        };
    }
    pub fn one() -> vec3 {
        return vec3 {
            source: [1.0f32,1.0f32,1.0f32]
        };
    }
    pub fn up() -> vec3 {
        return vec3{
            source: [0.0f32,1.0f32,0.0f32]
        };
    }
    pub fn down() -> vec3 {
        return vec3{
            source: [0.0f32,-1.0f32,0.0f32]
        };
    }
    pub fn left() -> vec3 {
        return vec3{
            source: [-1.0f32,0.0f32,0.0f32]
        };
    }
    pub fn right() -> vec3 {
        return vec3{
            source: [1.0f32,0.0f32,0.0f32]
        };
    }
    pub fn forward() -> vec3 {
        return vec3{
            source: [0.0f32,0.0f32,-1.0f32]
        };
    }
    pub fn back() -> vec3 {
        return vec3{
            source: [0.0f32,0.0f32,1.0f32]
        };
    }
    // constructor
    pub fn new(a: f32, b: f32, c: f32) -> vec3 {
        return vec3{
            source: [a, b, c]
        };
    }
    // members
    pub fn clone(&self) -> vec3{
        return vec3 { source: self.source};
    }
    pub fn mag(&self) -> f32{
        return self.sqr_mag().sqrt();
    }
    pub fn sqr_mag(&self) -> f32{
        return self[1]*self[1]+self[2]*self[2]+self[3]*self[3];
    }
    pub fn norm(&self) -> vec3 {
        return vec3::div(self, self.mag());
    }
    // statics
    pub fn angle(from: &vec3, to: &vec3) -> f32{
        let div = from.mag() * to.mag();
        let dot = vec3::dot(&from, &to);
        if div == 0.0f32 || dot == 0.0f32 {
            return self::PI;
        }
        let da = dot / div;
        return da.acos();
    }
    pub fn cross(lhs: &vec3, rhs: &vec3) -> vec3{
        return vec3::new(
            lhs[2] * rhs[3] - lhs[3] * rhs[2],
            lhs[1] * rhs[3] - lhs[3] * rhs[1],
            lhs[1] * rhs[2] - lhs[2] * rhs[1]
        );
    }
    pub fn dot(lhs: &vec3, rhs: &vec3) -> f32{
        return lhs[1]*rhs[1] + lhs[2]+rhs[2] * lhs[3]+rhs[3];
    }
    pub fn distance(lhs: &vec3, rhs: &vec3) -> f32{
        return vec3::sub(&rhs, &lhs).mag();
    }
    pub fn add(lhs: &vec3, rhs: &vec3) -> vec3 {
        return vec3 { source: [ lhs[1]+rhs[1],lhs[2]+rhs[2],lhs[3]+rhs[3] ] };
    }
    pub fn sub(lhs: &vec3, rhs: &vec3) -> vec3 {
        return vec3 { source: [ lhs[1]-rhs[1],lhs[2]-rhs[2],lhs[3]-rhs[3] ] };
    }
    pub fn mul(lhs: &vec3, factor: f32) -> vec3 {
        return vec3 { source: [ lhs[1]*factor,lhs[2]*factor,lhs[3]*factor ] };
    }
    pub fn div(lhs: &vec3, factor: f32) -> vec3 {
        return vec3 { source: [ lhs[1]/factor,lhs[2]/factor,lhs[3]/factor ] };
    }
    pub fn equals(lhs: &vec3, rhs: &vec3) -> bool {
        return if lhs.source == rhs.source {true} else {false};
    }
    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}

