use std;
use PI;

pub struct vec3 {
    pub source: [f32; 3]
}

impl std::ops::Add for vec3{
    type Output = vec3;

    fn add(self, other: vec3) -> vec3 {
        vec3::add(&self, &other)
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
        return self.source[0]*self.source[0]+self.source[1]*self.source[1]+self.source[2]*self.source[2];
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
            lhs.source[1] * rhs.source[2] - lhs.source[2] * rhs.source[1],
            lhs.source[0] * rhs.source[2] - lhs.source[2] * rhs.source[0],
            lhs.source[0] * rhs.source[1] - lhs.source[1] * rhs.source[0]
        );
    }
    pub fn dot(lhs: &vec3, rhs: &vec3) -> f32{
        return lhs.source[0] * rhs.source[0]+lhs.source[1] * rhs.source[1]+lhs.source[2] * rhs.source[2];
    }
    pub fn distance(lhs: &vec3, rhs: &vec3) -> f32{
        return vec3::sub(&rhs, &lhs).mag();
    }
    pub fn add(lhs: &vec3, rhs: &vec3) -> vec3 {
        return vec3 { source: [ lhs.source[0]+rhs.source[0],lhs.source[1]+rhs.source[1],lhs.source[2]+rhs.source[2] ] };
    }
    pub fn sub(lhs: &vec3, rhs: &vec3) -> vec3 {
        return vec3 { source: [ lhs.source[0]-rhs.source[0],lhs.source[1]-rhs.source[1],lhs.source[2]-rhs.source[2] ] };
    }
    pub fn mul(lhs: &vec3, factor: f32) -> vec3 {
        return vec3 { source: [ lhs.source[0]*factor,lhs.source[1]*factor,lhs.source[2]*factor ] };
    }
    pub fn div(lhs: &vec3, factor: f32) -> vec3 {
        return vec3 { source: [ lhs.source[0]/factor,lhs.source[1]/factor,lhs.source[2]/factor ] };
    }
    pub fn equals(lhs: &vec3, rhs: &vec3) -> bool {
        return if lhs.source == rhs.source {true} else {false};
    }
    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}

