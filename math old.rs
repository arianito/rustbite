
pub struct vec2 {
    pub x: f32,
    pub y: f32
}




pub struct vec3 {
    pub data: [f32; 3]
}


impl vec3 {
    pub fn angle(from: &vec3, to: &vec3) -> f32{
        let div = from.magnitude() * to.magnitude();
        let dot = vec3::dot(&from, &to);
        if div == 0.0 || dot == 0.0 {
            return std::f32::consts::PI;
        }
        let da = dot / div;
        return da.acos();
    }

    pub fn dot(lhs: &vec3, rhs: &vec3) -> f32{
        return lhs.data[0] * rhs.data[0] + lhs.data[1] * rhs.data[1] + lhs.data[2] * rhs.data[2];
    }

    pub fn cross(lhs: &vec3, rhs: &vec3) -> vec3{
        return vec3::new(
            lhs.data[1] * rhs.data[2] - lhs.data[2] * rhs.data[1],
            lhs.data[0] * rhs.data[2] - lhs.data[2] * rhs.data[0],
            lhs.data[0] * rhs.data[1] - lhs.data[1] * rhs.data[0]
        );
    }

    pub fn distance(lhs: &vec3, rhs: &vec3) -> f32{
        return vec3::sub(&rhs, &lhs).magnitude();
    }


    pub fn zero() -> vec3 {
        return vec3{
            data: [0.0, 0.0, 0.0]
        };
    }
    pub fn one() -> vec3 {
        return vec3{
            data: [1.0, 1.0, 1.0]
        };
    }
    pub fn up() -> vec3 {
        return vec3{
            data: [0.0, 1.0, 0.0]
        };
    }
    pub fn down() -> vec3 {
        return vec3{
            data: [0.0, -1.0, 0.0]
        };
    }
    pub fn forward() -> vec3 {
        return vec3{
            data: [0.0, 0.0, 1.0]
        };
    }
    pub fn back() -> vec3 {
        return vec3{
            data: [0.0, 0.0, -1.0]
        };
    }
    pub fn left() -> vec3 {
        return vec3{
            data: [-1.0, 0.0, 0.0]
        };
    }
    pub fn right() -> vec3 {
        return vec3{
            data: [1.0, 0.0, 0.0]
        };
    }

    pub fn new(x: f32, y: f32, z: f32) -> vec3 {
        return vec3 {
            data: [x, y, z]
        };
    }

    pub fn x(&mut self) -> f32{
        return self.data[0];
    }

    pub fn y(&mut self) -> f32{
        return self.data[1];
    }

    pub fn z(&mut self) -> f32{
        return self.data[2];
    }

    pub fn clone(&self) -> vec3{
        return vec3 {
            data: self.data
        };
    }

    pub fn add(lhs: &vec3, rhs: &vec3) -> vec3 {
        let mut nw =  vec3::zero();
        for i in 0..3 {
            nw.data[i] = lhs.data[i] + rhs.data[i];
        }
        return nw;
    }

    pub fn sub(lhs: &vec3, rhs: &vec3) -> vec3 {
        let mut nw =  vec3::zero();
        for i in 0..3 {
            nw.data[i] = lhs.data[i] - rhs.data[i];
        }
        return nw;
    }

    pub fn mul(lhs: &vec3, multiplier: f32) -> vec3 {
        let mut nw =  vec3::zero();
        for i in 0..3 {
            nw.data[i] = lhs.data[i] * multiplier;
        }
        return nw;
    }


    pub fn div(lhs: &vec3, divider: f32) -> vec3 {
        let mut nw =  vec3::zero();
        for i in 0..3 {
            nw.data[i] = lhs.data[i] / divider;
        }
        return nw;
    }



    pub fn equals(lhs: &vec3, rhs: &vec3) -> bool {
        for i in 0..3 {
            if lhs.data[i] != rhs.data[i] {
                return false;
            }
        }
        return true;
    }



//
pub fn addBy(&mut self, rhs: &vec3) -> &mut vec3 {
    for i in 0..3 {
        self.data[i] = self.data[i] + rhs.data[i];
    }
    return self;
}

pub fn subBy(&mut self, rhs: &vec3) -> &mut vec3 {
    for i in 0..3 {
        self.data[i] = self.data[i] - rhs.data[i];
    }
    return self;
}

pub fn mulBy(&mut self, multiplier: f32) -> &mut vec3 {
    for i in 0..3 {
        self.data[i] = self.data[i] * multiplier;
    }
    return self;
}


pub fn divBy(&mut self, divider: f32) -> &mut vec3 {
    for i in 0..3 {
        self.data[i] = self.data[i] / divider;
    }
    return self;
}



pub fn equalsWith(&self, rhs: &vec3) -> bool {
    for i in 0..3 {
        if self.data[i] != rhs.data[i] {
            return false;
        }
    }
    return true;
}

//


    pub fn magnitude(&self) -> f32{
        let mut sum : f32= 0.0;
        for i in self.data.iter() {
            sum = sum + (i * i);
        }
        return sum.sqrt();
    }

    pub fn sqrMagnitude(&self) -> f32{
        let mut sum : f32= 0.0;
        for i in self.data.iter() {
            sum = sum + (i * i);
        }
        return sum;
    }
    pub fn normalized(&self) -> vec3 {
        let mag = self.magnitude();
        if mag == 0.0 {
            return vec3::zero();
        }

        let mut v = self.clone();
        v.divBy(mag);
        return v;
    }



    pub fn setXYZ(&mut self, x: f32, y: f32, z: f32) -> &mut vec3 {
        self.data[0] = x;
        self.data[1] = y;
        self.data[2] = z;
        return self;
    }

    pub fn setXY(&mut self, x: f32, y: f32) -> &mut vec3 {
        self.data[0] = x;
        self.data[1] = y;
        return self;
    }

    pub fn setX(&mut self, x: f32) -> &mut vec3 {
        self.data[0] = x;
        return self;
    }
    pub fn setY(&mut self, y: f32) -> &mut vec3 {
        self.data[1] = y;
        return self;
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}




pub struct mat4 {
    pub data : [[f32; 4]; 4]
}
impl mat4 {
    pub fn init(arr: [[f32; 4]; 4]) -> mat4 {
        return mat4{
            data: arr
        }
    }

    pub fn new(size: f32) -> mat4 {
        return mat4{
            data: [[size, 0.0, 0.0, 0.0],[0.0, size, 0.0, 0.0],[0.0, 0.0, size, 0.0],[0.0, 0.0, 0.0, size]]
        }
    }
    pub fn identify() -> mat4 {
        return mat4{
            data: [[1.0, 0.0, 0.0, 0.0],[0.0, 1.0, 0.0, 0.0],[0.0, 0.0, 1.0, 0.0],[0.0, 0.0, 0.0, 1.0]]
        }
    }
    pub fn zero() -> mat4 {
        return mat4{
            data: [[0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0]]
        }
    }


    pub fn translate(x: f32, y: f32, z: f32) -> mat4 {
        return mat4::init([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, 0.0, 0.0, 1.0],
        ]);
    }
    pub fn translateByVec(t: &vec3) -> mat4 {
        return mat4::init([
            [1.0, 0.0, 0.0, t.data[0]],
            [0.0, 1.0, 0.0, t.data[1]],
            [0.0, 0.0, 1.0, t.data[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn determinant(lhs: &mat4) -> f32 {
        return lhs.data[0][0] * lhs.data[1][1] * lhs.data[2][2] * lhs.data[3][3] -
        lhs.data[0][0] * lhs.data[1][1] * lhs.data[2][3] * lhs.data[3][2] -
        lhs.data[0][0] * lhs.data[1][2] * lhs.data[2][1] * lhs.data[3][3] +
        lhs.data[0][0] * lhs.data[1][2] * lhs.data[2][3] * lhs.data[3][1] +
        lhs.data[0][0] * lhs.data[1][3] * lhs.data[2][1] * lhs.data[3][2] -
        lhs.data[0][0] * lhs.data[1][3] * lhs.data[2][2] * lhs.data[3][1] -
        lhs.data[0][1] * lhs.data[1][0] * lhs.data[2][2] * lhs.data[3][3] +
        lhs.data[0][1] * lhs.data[1][0] * lhs.data[2][3] * lhs.data[3][2] +
        lhs.data[0][1] * lhs.data[1][2] * lhs.data[2][0] * lhs.data[3][3] -
        lhs.data[0][1] * lhs.data[1][2] * lhs.data[2][3] * lhs.data[3][0] -
        lhs.data[0][1] * lhs.data[1][3] * lhs.data[2][0] * lhs.data[3][2] +
        lhs.data[0][1] * lhs.data[1][3] * lhs.data[2][2] * lhs.data[3][0] +
        lhs.data[0][2] * lhs.data[1][0] * lhs.data[2][1] * lhs.data[3][3] -
        lhs.data[0][2] * lhs.data[1][0] * lhs.data[2][3] * lhs.data[3][1] -
        lhs.data[0][2] * lhs.data[1][1] * lhs.data[2][0] * lhs.data[3][3] +
        lhs.data[0][2] * lhs.data[1][1] * lhs.data[2][3] * lhs.data[3][0] +
        lhs.data[0][2] * lhs.data[1][3] * lhs.data[2][0] * lhs.data[3][1] -
        lhs.data[0][2] * lhs.data[1][3] * lhs.data[2][1] * lhs.data[3][0] -
        lhs.data[0][3] * lhs.data[1][0] * lhs.data[2][1] * lhs.data[3][2] +
        lhs.data[0][3] * lhs.data[1][0] * lhs.data[2][2] * lhs.data[3][1] +
        lhs.data[0][3] * lhs.data[1][1] * lhs.data[2][0] * lhs.data[3][2] -
        lhs.data[0][3] * lhs.data[1][1] * lhs.data[2][2] * lhs.data[3][0] -
        lhs.data[0][3] * lhs.data[1][2] * lhs.data[2][0] * lhs.data[3][1] +
        lhs.data[0][3] * lhs.data[1][2] * lhs.data[2][1] * lhs.data[3][0];
    }

    pub fn transpose(lhs: &mat4) -> mat4 {
        return mat4::init([
            [lhs.data[0][0], lhs.data[1][0], lhs.data[2][0], lhs.data[3][0]],
            [lhs.data[0][1], lhs.data[1][1], lhs.data[2][1], lhs.data[3][1]],
            [lhs.data[0][2], lhs.data[1][2], lhs.data[2][2], lhs.data[3][2]],
            [lhs.data[0][3], lhs.data[1][3], lhs.data[2][3], lhs.data[3][3]],
        ]);
    }

    pub fn inverse(lhs: &mat4) -> mat4 {
        let mut m = mat4::identify();
        let det = mat4::determinant(&lhs);
        if det == 0.0 {
            return m;
        }
        m.data = [[
        (lhs.data[1][1] * lhs.data[2][2] * lhs.data[3][3] + lhs.data[1][2] * lhs.data[2][3] * lhs.data[3][1] +
        lhs.data[1][3] * lhs.data[2][1] * lhs.data[3][2] - lhs.data[1][1] * lhs.data[2][3] * lhs.data[3][2] -
        lhs.data[1][2] * lhs.data[2][1] * lhs.data[3][3] - lhs.data[1][3] * lhs.data[2][2] * lhs.data[3][1])/det,
        (lhs.data[0][1] * lhs.data[2][3] * lhs.data[3][2] + lhs.data[0][2] * lhs.data[2][1] * lhs.data[3][3] +
        lhs.data[0][3] * lhs.data[2][2] * lhs.data[3][1] - lhs.data[0][1] * lhs.data[2][2] * lhs.data[3][3] -
        lhs.data[0][2] * lhs.data[2][3] * lhs.data[3][1] - lhs.data[0][3] * lhs.data[2][1] * lhs.data[3][2])/det,
        (lhs.data[0][1] * lhs.data[1][2] * lhs.data[3][3] + lhs.data[0][2] * lhs.data[1][3] * lhs.data[3][1] +
        lhs.data[0][3] * lhs.data[1][1] * lhs.data[3][2] - lhs.data[0][1] * lhs.data[1][3] * lhs.data[3][2] -
        lhs.data[0][2] * lhs.data[1][1] * lhs.data[3][3] - lhs.data[0][3] * lhs.data[1][2] * lhs.data[3][1])/det,
        (lhs.data[0][1] * lhs.data[1][3] * lhs.data[2][2] + lhs.data[0][2] * lhs.data[1][1] * lhs.data[2][3] +
        lhs.data[0][3] * lhs.data[1][2] * lhs.data[2][1] - lhs.data[0][1] * lhs.data[1][2] * lhs.data[2][3] -
        lhs.data[0][2] * lhs.data[1][3] * lhs.data[2][1] - lhs.data[0][3] * lhs.data[1][1] * lhs.data[2][2])/det],
        [(lhs.data[1][0] * lhs.data[2][3] * lhs.data[3][2] + lhs.data[1][2] * lhs.data[2][0] * lhs.data[3][3] +
        lhs.data[1][3] * lhs.data[2][2] * lhs.data[3][0] - lhs.data[1][0] * lhs.data[2][2] * lhs.data[3][3] -
        lhs.data[1][2] * lhs.data[2][3] * lhs.data[3][0] - lhs.data[1][3] * lhs.data[2][0] * lhs.data[3][2])/det,
        (lhs.data[0][0] * lhs.data[2][2] * lhs.data[3][3] + lhs.data[0][2] * lhs.data[2][3] * lhs.data[3][0] +
        lhs.data[0][3] * lhs.data[2][0] * lhs.data[3][2] - lhs.data[0][0] * lhs.data[2][3] * lhs.data[3][2] -
        lhs.data[0][2] * lhs.data[2][0] * lhs.data[3][3] - lhs.data[0][3] * lhs.data[2][2] * lhs.data[3][0])/det,
        (lhs.data[0][0] * lhs.data[1][3] * lhs.data[3][2] + lhs.data[0][2] * lhs.data[1][0] * lhs.data[3][3] +
        lhs.data[0][3] * lhs.data[1][2] * lhs.data[3][0] - lhs.data[0][0] * lhs.data[1][2] * lhs.data[3][3] -
        lhs.data[0][2] * lhs.data[1][3] * lhs.data[3][0] - lhs.data[0][3] * lhs.data[1][0] * lhs.data[3][2])/det,
        (lhs.data[0][0] * lhs.data[1][2] * lhs.data[2][3] + lhs.data[0][2] * lhs.data[1][3] * lhs.data[2][0] +
        lhs.data[0][3] * lhs.data[1][0] * lhs.data[2][2] - lhs.data[0][0] * lhs.data[1][3] * lhs.data[2][2] -
        lhs.data[0][2] * lhs.data[1][0] * lhs.data[2][3] - lhs.data[0][3] * lhs.data[1][2] * lhs.data[2][0])/det],
        [(lhs.data[1][0] * lhs.data[2][1] * lhs.data[3][3] + lhs.data[1][1] * lhs.data[2][3] * lhs.data[3][0] +
        lhs.data[1][3] * lhs.data[2][0] * lhs.data[3][1] - lhs.data[1][0] * lhs.data[2][3] * lhs.data[3][1] -
        lhs.data[1][1] * lhs.data[2][0] * lhs.data[3][3] - lhs.data[1][3] * lhs.data[2][1] * lhs.data[3][0])/det,
        (lhs.data[0][0] * lhs.data[2][3] * lhs.data[3][1] + lhs.data[0][1] * lhs.data[2][0] * lhs.data[3][3] +
        lhs.data[0][3] * lhs.data[2][1] * lhs.data[3][0] - lhs.data[0][0] * lhs.data[2][1] * lhs.data[3][3] -
        lhs.data[0][1] * lhs.data[2][3] * lhs.data[3][0] - lhs.data[0][3] * lhs.data[2][0] * lhs.data[3][1])/det,
        (lhs.data[0][0] * lhs.data[1][1] * lhs.data[3][3] + lhs.data[0][1] * lhs.data[1][3] * lhs.data[3][0] +
        lhs.data[0][3] * lhs.data[1][0] * lhs.data[3][1] - lhs.data[0][0] * lhs.data[1][3] * lhs.data[3][1] -
        lhs.data[0][1] * lhs.data[1][0] * lhs.data[3][3] - lhs.data[0][3] * lhs.data[1][1] * lhs.data[3][0])/det,
        (lhs.data[0][0] * lhs.data[1][3] * lhs.data[2][1] + lhs.data[0][1] * lhs.data[1][0] * lhs.data[2][3] +
        lhs.data[0][3] * lhs.data[1][1] * lhs.data[2][0] - lhs.data[0][0] * lhs.data[1][1] * lhs.data[2][3] -
        lhs.data[0][1] * lhs.data[1][3] * lhs.data[2][0] - lhs.data[0][3] * lhs.data[1][0] * lhs.data[2][1])/det],
        [(lhs.data[1][0] * lhs.data[2][2] * lhs.data[3][1] + lhs.data[1][1] * lhs.data[2][0] * lhs.data[3][2] +
        lhs.data[1][2] * lhs.data[2][1] * lhs.data[3][0] - lhs.data[1][0] * lhs.data[2][1] * lhs.data[3][2] -
        lhs.data[1][1] * lhs.data[2][2] * lhs.data[3][0] - lhs.data[1][2] * lhs.data[2][0] * lhs.data[3][1])/det,
        (lhs.data[0][0] * lhs.data[2][1] * lhs.data[3][2] + lhs.data[0][1] * lhs.data[2][2] * lhs.data[3][0] +
        lhs.data[0][2] * lhs.data[2][0] * lhs.data[3][1] - lhs.data[0][0] * lhs.data[2][2] * lhs.data[3][1] -
        lhs.data[0][1] * lhs.data[2][0] * lhs.data[3][2] - lhs.data[0][2] * lhs.data[2][1] * lhs.data[3][0])/det,
        (lhs.data[0][0] * lhs.data[1][2] * lhs.data[3][1] + lhs.data[0][1] * lhs.data[1][0] * lhs.data[3][2] +
        lhs.data[0][2] * lhs.data[1][1] * lhs.data[3][0] - lhs.data[0][0] * lhs.data[1][1] * lhs.data[3][2] -
        lhs.data[0][1] * lhs.data[1][2] * lhs.data[3][0] - lhs.data[0][2] * lhs.data[1][0] * lhs.data[3][1])/det,
        (lhs.data[0][0] * lhs.data[1][1] * lhs.data[2][2] + lhs.data[0][1] * lhs.data[1][2] * lhs.data[2][0] +
        lhs.data[0][2] * lhs.data[1][0] * lhs.data[2][1] - lhs.data[0][0] * lhs.data[1][2] * lhs.data[2][1] -
        lhs.data[0][1] * lhs.data[1][0] * lhs.data[2][2] - lhs.data[0][2] * lhs.data[1][1] * lhs.data[2][0])/det]];
        return m;
    }


    pub fn mul(lhs: &mat4, rhs: &mat4) -> mat4 {

        return mat4::init([
            [
                lhs.data[0][0]*rhs.data[0][0] + lhs.data[0][1]*rhs.data[1][0] + lhs.data[0][2]*rhs.data[2][0] + lhs.data[0][3]*rhs.data[3][0],
                lhs.data[0][0]*rhs.data[0][1] + lhs.data[0][1]*rhs.data[1][1] + lhs.data[0][2]*rhs.data[2][1] + lhs.data[0][3]*rhs.data[3][1],
                lhs.data[0][0]*rhs.data[0][2] + lhs.data[0][1]*rhs.data[1][2] + lhs.data[0][2]*rhs.data[2][2] + lhs.data[0][3]*rhs.data[3][2],
                lhs.data[0][0]*rhs.data[0][3] + lhs.data[0][1]*rhs.data[1][3] + lhs.data[0][2]*rhs.data[2][3] + lhs.data[0][3]*rhs.data[3][3],
            ],
            [
                lhs.data[1][0]*rhs.data[0][0] + lhs.data[1][1]*rhs.data[1][0] + lhs.data[1][2]*rhs.data[2][0] + lhs.data[1][3]*rhs.data[3][0],
                lhs.data[1][0]*rhs.data[0][1] + lhs.data[1][1]*rhs.data[1][1] + lhs.data[1][2]*rhs.data[2][1] + lhs.data[1][3]*rhs.data[3][1],
                lhs.data[1][0]*rhs.data[0][2] + lhs.data[1][1]*rhs.data[1][2] + lhs.data[1][2]*rhs.data[2][2] + lhs.data[1][3]*rhs.data[3][2],
                lhs.data[1][0]*rhs.data[0][3] + lhs.data[1][1]*rhs.data[1][3] + lhs.data[1][2]*rhs.data[2][3] + lhs.data[1][3]*rhs.data[3][3],
            ],
            [
                lhs.data[2][0]*rhs.data[0][0] + lhs.data[2][1]*rhs.data[1][0] + lhs.data[2][2]*rhs.data[2][0] + lhs.data[2][3]*rhs.data[3][0],
                lhs.data[2][0]*rhs.data[0][1] + lhs.data[2][1]*rhs.data[1][1] + lhs.data[2][2]*rhs.data[2][1] + lhs.data[2][3]*rhs.data[3][1],
                lhs.data[2][0]*rhs.data[0][2] + lhs.data[2][1]*rhs.data[1][2] + lhs.data[2][2]*rhs.data[2][2] + lhs.data[2][3]*rhs.data[3][2],
                lhs.data[2][0]*rhs.data[0][3] + lhs.data[2][1]*rhs.data[1][3] + lhs.data[2][2]*rhs.data[2][3] + lhs.data[2][3]*rhs.data[3][3],
            ],
            [
                lhs.data[3][0]*rhs.data[0][0] + lhs.data[3][1]*rhs.data[1][0] + lhs.data[3][2]*rhs.data[2][0] + lhs.data[3][3]*rhs.data[3][0],
                lhs.data[3][0]*rhs.data[0][1] + lhs.data[3][1]*rhs.data[1][1] + lhs.data[3][2]*rhs.data[2][1] + lhs.data[3][3]*rhs.data[3][1],
                lhs.data[3][0]*rhs.data[0][2] + lhs.data[3][1]*rhs.data[1][2] + lhs.data[3][2]*rhs.data[2][2] + lhs.data[3][3]*rhs.data[3][2],
                lhs.data[3][0]*rhs.data[0][3] + lhs.data[3][1]*rhs.data[1][3] + lhs.data[3][2]*rhs.data[2][3] + lhs.data[3][3]*rhs.data[3][3],
            ],
        ]);
    }

    pub fn div(lhs: &mat4, rhs: &mat4) -> mat4 {
        let m2 = mat4::inverse(&rhs);
        return mat4::mul(&lhs, &m2);
    }


    pub fn equals(lhs: &mat4, rhs: &mat4) -> bool{
        for i in 0..4{
            for j in 0..4{
                if lhs.data[i][j] != rhs.data[i][j]{
                    return false;
                }
            }
        }
        return true;
    }


    pub fn clone(&self) -> mat4{
        return mat4::init(
            self.data
        );
    }

    pub fn det(&self) -> f32 {
        return mat4::determinant(&self)
    }

    pub fn trans(&mut self) -> mat4 {
        return mat4::transpose(&self);
    }

    pub fn inv(&self) -> mat4 {
        return mat4::inverse(&self);
    }


    pub fn mulBy(&mut self, rhs: &mat4) -> &mut mat4{
        self.data = mat4::mul(&self, &rhs).data;
        return self;
    }

    pub fn divBy(&mut self, rhs: &mat4) -> &mut mat4{
        self.data = mat4::div(&self, &rhs).data;
        return self;
    }


    pub fn equalsWith(&self, rhs: &mat4) -> bool{
        return mat4::equals(self, &rhs);
    }


    pub fn isIdentity(&self) -> bool{
        let b = mat4::identify();
        return mat4::equals(&self, &b);
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}
