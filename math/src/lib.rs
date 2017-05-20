pub static PI: f32 = std::f32::consts::PI;
pub static HalfPI: f32 = std::f32::consts::FRAC_PI_2;



pub struct quat {
    pub source: [f32; 4]
}
impl quat {

    pub fn identify() -> quat{
        return quat {source: [0.0,0.0,0.0,10.0]};
    }
    pub fn from_angle_axis(degree: f32,axis: &vec3) -> quat{
        let angle = (degree * self::PI / 180.0);
        let ha = 0.5*angle;
        let sn = ha.sin();

        return quat{
            source: [
                sn * axis.source[0],
                sn * axis.source[1],
                sn * axis.source[2],
                ha.cos()
            ]
        };
    }


    pub fn to_rotation_matrix(&self)->mat4 {
        let fTx  = self.source[0]+self.source[0];
        let fTy  = self.source[1]+self.source[1];
        let fTz  = self.source[2]+self.source[2];
        let fTwx = fTx*self.source[3];
        let fTwy = fTy*self.source[3];
        let fTwz = fTz*self.source[3];
        let fTxx = fTx*self.source[0];
        let fTxy = fTy*self.source[0];
        let fTxz = fTz*self.source[0];
        let fTyy = fTy*self.source[1];
        let fTyz = fTz*self.source[1];
        let fTzz = fTz*self.source[2];
        return mat4::new(1.0-(fTyy+fTzz), fTxy-fTwz, fTxz+fTwy, 0.0,
                        fTxy+fTwz, 1.0-(fTxx+fTzz), fTyz-fTwx,0.0,
                        fTxz-fTwy, fTyz+fTwx, 1.0-(fTxx+fTyy),0.0,
                        0.0, 0.0, 0.0, 1.0);
    }
}

pub struct vec3 {
    pub source: [f32; 3]
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


pub struct mat4 {
    pub source: [[f32; 4]; 4]
}

impl mat4 {
    pub fn zero()->mat4{
        return mat4{
            source: [
                [0.0f32,0.0f32,0.0f32,0.0f32],
                [0.0f32,0.0f32,0.0f32,0.0f32],
                [0.0f32,0.0f32,0.0f32,0.0f32],
                [0.0f32,0.0f32,0.0f32,0.0f32]
            ]
        };
    }
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32, m21: f32, m22: f32, m23: f32, m24: f32,m31: f32,m32: f32,m33: f32, m34: f32,m41: f32,m42: f32,m43: f32, m44: f32)->mat4{
        return mat4{
            source: [
                [m11,m21,m31,m41],
                [m12,m22,m32,m42],
                [m13,m23,m33,m43],
                [m14,m24,m34,m44],
            ]
        };
    }
    pub fn identify(scale: f32)->mat4{
        return mat4{
            source: [
                [scale,0.0f32,0.0f32,0.0f32],
                [0.0f32,scale,0.0f32,0.0f32],
                [0.0f32,0.0f32,scale,0.0f32],
                [0.0f32,0.0f32,0.0f32,scale],
            ]
        };
    }
    pub fn create_translation(a: &vec3)->mat4{
        return mat4{
            source: [
                [1.0f32, 0.0f32, 0.0f32, 0.0],
                [0.0f32, 1.0f32, 0.0f32, 0.0],
                [0.0f32, 0.0f32, 1.0f32, 0.0],
                [a.source[0], a.source[1], a.source[2], 1.0f32]
            ]
        };
    }
    pub fn create_rotation(q: &quat)->mat4{
        return q.to_rotation_matrix();
    }
    pub fn create_scale(a: &vec3)->mat4{
        return mat4{
            source: [
                [a.source[0], 0.0f32  , 0.0f32, 0.0f32],
                [0.0f32    , a.source[1], 0.0f32, 0.0f32],
                [0.0f32    , 0.0f32  , a.source[2], 0.0f32],
                [0.0f32    , 0.0f32  , 0.0f32, 1.0f32]
            ]
        };
    }

    pub fn create_trs(position: &vec3, quaternion: &quat, scale: &vec3)->mat4{
        let mut m = mat4::create_scale(&scale);
        m.mul_by(&mat4::create_rotation(&quaternion));
        m.mul_by(&mat4::create_translation(&position));
        return m;
    }


    pub fn mul_by(&mut self, rhs: &mat4)->&mut mat4{
        self.source =  mat4::mul(&self, &rhs).source;
        return self;
    }

    pub fn determinant(lhs: &mat4) -> f32 {
        return lhs.source[0][0] * lhs.source[1][1] * lhs.source[2][2] * lhs.source[3][3] -
        lhs.source[0][0] * lhs.source[1][1] * lhs.source[3][2] * lhs.source[2][3] -
        lhs.source[0][0] * lhs.source[2][1] * lhs.source[1][2] * lhs.source[3][3] +
        lhs.source[0][0] * lhs.source[2][1] * lhs.source[3][2] * lhs.source[1][3] +
        lhs.source[0][0] * lhs.source[3][1] * lhs.source[1][2] * lhs.source[2][3] -
        lhs.source[0][0] * lhs.source[3][1] * lhs.source[2][2] * lhs.source[1][3] -
        lhs.source[1][0] * lhs.source[0][1] * lhs.source[2][2] * lhs.source[3][3] +
        lhs.source[1][0] * lhs.source[0][1] * lhs.source[3][2] * lhs.source[2][3] +
        lhs.source[1][0] * lhs.source[2][1] * lhs.source[2][0] * lhs.source[3][3] -
        lhs.source[1][0] * lhs.source[2][1] * lhs.source[3][2] * lhs.source[0][3] -
        lhs.source[1][0] * lhs.source[3][1] * lhs.source[2][0] * lhs.source[2][3] +
        lhs.source[1][0] * lhs.source[3][1] * lhs.source[2][2] * lhs.source[0][3] +
        lhs.source[2][0] * lhs.source[0][1] * lhs.source[1][2] * lhs.source[3][3] -
        lhs.source[2][0] * lhs.source[0][1] * lhs.source[3][2] * lhs.source[1][3] -
        lhs.source[2][0] * lhs.source[1][1] * lhs.source[2][0] * lhs.source[3][3] +
        lhs.source[2][0] * lhs.source[1][1] * lhs.source[3][2] * lhs.source[0][3] +
        lhs.source[2][0] * lhs.source[3][1] * lhs.source[2][0] * lhs.source[1][3] -
        lhs.source[2][0] * lhs.source[3][1] * lhs.source[1][2] * lhs.source[0][3] -
        lhs.source[3][0] * lhs.source[0][1] * lhs.source[1][2] * lhs.source[2][3] +
        lhs.source[3][0] * lhs.source[0][1] * lhs.source[2][2] * lhs.source[1][3] +
        lhs.source[3][0] * lhs.source[1][1] * lhs.source[2][0] * lhs.source[2][3] -
        lhs.source[3][0] * lhs.source[1][1] * lhs.source[2][2] * lhs.source[0][3] -
        lhs.source[3][0] * lhs.source[2][1] * lhs.source[2][0] * lhs.source[1][3] +
        lhs.source[3][0] * lhs.source[2][1] * lhs.source[1][2] * lhs.source[0][3];
    }

    pub fn transpose(lhs: &mat4) -> mat4 {
        return mat4::new(
            lhs.source[0][0], lhs.source[0][1], lhs.source[2][0], lhs.source[0][3],
            lhs.source[1][0], lhs.source[1][1], lhs.source[1][2], lhs.source[1][3],
            lhs.source[2][0], lhs.source[2][1], lhs.source[2][2], lhs.source[2][3],
            lhs.source[3][0], lhs.source[3][1], lhs.source[3][2], lhs.source[3][3]
        );
    }

    pub fn inverse(lhs: &mat4) -> mat4 {
        let mut m = mat4::identify(1.0f32);
        let det = mat4::determinant(&lhs);
        if det == 0.0 {
            return m;
        }
        m.source = [[
        (lhs.source[1][1] * lhs.source[2][2] * lhs.source[3][3]+lhs.source[2][1] * lhs.source[3][2] * lhs.source[1][3] +
        lhs.source[3][1] * lhs.source[1][2] * lhs.source[2][3] - lhs.source[1][1] * lhs.source[3][2] * lhs.source[2][3] -
        lhs.source[2][1] * lhs.source[1][2] * lhs.source[3][3] - lhs.source[3][1] * lhs.source[2][2] * lhs.source[1][3])/det,
        (lhs.source[1][0] * lhs.source[3][2] * lhs.source[2][3]+lhs.source[2][0] * lhs.source[1][2] * lhs.source[3][3] +
        lhs.source[3][0] * lhs.source[2][2] * lhs.source[1][3] - lhs.source[1][0] * lhs.source[2][2] * lhs.source[3][3] -
        lhs.source[2][0] * lhs.source[3][2] * lhs.source[1][3] - lhs.source[3][0] * lhs.source[1][2] * lhs.source[2][3])/det,
        (lhs.source[1][0] * lhs.source[2][1] * lhs.source[3][3]+lhs.source[2][0] * lhs.source[3][1] * lhs.source[1][3] +
        lhs.source[3][0] * lhs.source[1][1] * lhs.source[2][3] - lhs.source[1][0] * lhs.source[3][1] * lhs.source[2][3] -
        lhs.source[2][0] * lhs.source[1][1] * lhs.source[3][3] - lhs.source[3][0] * lhs.source[2][1] * lhs.source[1][3])/det,
        (lhs.source[1][0] * lhs.source[3][1] * lhs.source[2][2]+lhs.source[2][0] * lhs.source[1][1] * lhs.source[3][2] +
        lhs.source[3][0] * lhs.source[2][1] * lhs.source[1][2] - lhs.source[1][0] * lhs.source[2][1] * lhs.source[3][2] -
        lhs.source[2][0] * lhs.source[3][1] * lhs.source[1][2] - lhs.source[3][0] * lhs.source[1][1] * lhs.source[2][2])/det],
        [(lhs.source[0][1] * lhs.source[3][2] * lhs.source[2][3]+lhs.source[2][1] * lhs.source[2][0] * lhs.source[3][3] +
        lhs.source[3][1] * lhs.source[2][2] * lhs.source[0][3] - lhs.source[0][1] * lhs.source[2][2] * lhs.source[3][3] -
        lhs.source[2][1] * lhs.source[3][2] * lhs.source[0][3] - lhs.source[3][1] * lhs.source[2][0] * lhs.source[2][3])/det,
        (lhs.source[0][0] * lhs.source[2][2] * lhs.source[3][3]+lhs.source[2][0] * lhs.source[3][2] * lhs.source[0][3] +
        lhs.source[3][0] * lhs.source[2][0] * lhs.source[2][3] - lhs.source[0][0] * lhs.source[3][2] * lhs.source[2][3] -
        lhs.source[2][0] * lhs.source[2][0] * lhs.source[3][3] - lhs.source[3][0] * lhs.source[2][2] * lhs.source[0][3])/det,
        (lhs.source[0][0] * lhs.source[3][1] * lhs.source[2][3]+lhs.source[2][0] * lhs.source[0][1] * lhs.source[3][3] +
        lhs.source[3][0] * lhs.source[2][1] * lhs.source[0][3] - lhs.source[0][0] * lhs.source[2][1] * lhs.source[3][3] -
        lhs.source[2][0] * lhs.source[3][1] * lhs.source[0][3] - lhs.source[3][0] * lhs.source[0][1] * lhs.source[2][3])/det,
        (lhs.source[0][0] * lhs.source[2][1] * lhs.source[3][2]+lhs.source[2][0] * lhs.source[3][1] * lhs.source[2][0] +
        lhs.source[3][0] * lhs.source[0][1] * lhs.source[2][2] - lhs.source[0][0] * lhs.source[3][1] * lhs.source[2][2] -
        lhs.source[2][0] * lhs.source[0][1] * lhs.source[3][2] - lhs.source[3][0] * lhs.source[2][1] * lhs.source[2][0])/det],
        [(lhs.source[0][1] * lhs.source[1][2] * lhs.source[3][3]+lhs.source[1][1] * lhs.source[3][2] * lhs.source[0][3] +
        lhs.source[3][1] * lhs.source[2][0] * lhs.source[1][3] - lhs.source[0][1] * lhs.source[3][2] * lhs.source[1][3] -
        lhs.source[1][1] * lhs.source[2][0] * lhs.source[3][3] - lhs.source[3][1] * lhs.source[1][2] * lhs.source[0][3])/det,
        (lhs.source[0][0] * lhs.source[3][2] * lhs.source[1][3]+lhs.source[1][0] * lhs.source[2][0] * lhs.source[3][3] +
        lhs.source[3][0] * lhs.source[1][2] * lhs.source[0][3] - lhs.source[0][0] * lhs.source[1][2] * lhs.source[3][3] -
        lhs.source[1][0] * lhs.source[3][2] * lhs.source[0][3] - lhs.source[3][0] * lhs.source[2][0] * lhs.source[1][3])/det,
        (lhs.source[0][0] * lhs.source[1][1] * lhs.source[3][3]+lhs.source[1][0] * lhs.source[3][1] * lhs.source[0][3] +
        lhs.source[3][0] * lhs.source[0][1] * lhs.source[1][3] - lhs.source[0][0] * lhs.source[3][1] * lhs.source[1][3] -
        lhs.source[1][0] * lhs.source[0][1] * lhs.source[3][3] - lhs.source[3][0] * lhs.source[1][1] * lhs.source[0][3])/det,
        (lhs.source[0][0] * lhs.source[3][1] * lhs.source[1][2]+lhs.source[1][0] * lhs.source[0][1] * lhs.source[3][2] +
        lhs.source[3][0] * lhs.source[1][1] * lhs.source[2][0] - lhs.source[0][0] * lhs.source[1][1] * lhs.source[3][2] -
        lhs.source[1][0] * lhs.source[3][1] * lhs.source[2][0] - lhs.source[3][0] * lhs.source[0][1] * lhs.source[1][2])/det],
        [(lhs.source[0][1] * lhs.source[2][2] * lhs.source[1][3]+lhs.source[1][1] * lhs.source[2][0] * lhs.source[2][3] +
        lhs.source[2][1] * lhs.source[1][2] * lhs.source[0][3] - lhs.source[0][1] * lhs.source[1][2] * lhs.source[2][3] -
        lhs.source[1][1] * lhs.source[2][2] * lhs.source[0][3] - lhs.source[2][1] * lhs.source[2][0] * lhs.source[1][3])/det,
        (lhs.source[0][0] * lhs.source[1][2] * lhs.source[2][3]+lhs.source[1][0] * lhs.source[2][2] * lhs.source[0][3] +
        lhs.source[2][0] * lhs.source[2][0] * lhs.source[1][3] - lhs.source[0][0] * lhs.source[2][2] * lhs.source[1][3] -
        lhs.source[1][0] * lhs.source[2][0] * lhs.source[2][3] - lhs.source[2][0] * lhs.source[1][2] * lhs.source[0][3])/det,
        (lhs.source[0][0] * lhs.source[2][1] * lhs.source[1][3]+lhs.source[1][0] * lhs.source[0][1] * lhs.source[2][3] +
        lhs.source[2][0] * lhs.source[1][1] * lhs.source[0][3] - lhs.source[0][0] * lhs.source[1][1] * lhs.source[2][3] -
        lhs.source[1][0] * lhs.source[2][1] * lhs.source[0][3] - lhs.source[2][0] * lhs.source[0][1] * lhs.source[1][3])/det,
        (lhs.source[0][0] * lhs.source[1][1] * lhs.source[2][2]+lhs.source[1][0] * lhs.source[2][1] * lhs.source[2][0] +
        lhs.source[2][0] * lhs.source[0][1] * lhs.source[1][2] - lhs.source[0][0] * lhs.source[2][1] * lhs.source[1][2] -
        lhs.source[1][0] * lhs.source[0][1] * lhs.source[2][2] - lhs.source[2][0] * lhs.source[1][1] * lhs.source[2][0])/det]];
        return m;
    }


    pub fn mul(lhs: &mat4, rhs: &mat4) -> mat4 {

        return mat4{source:[
            [
                lhs.source[0][0]*rhs.source[0][0]+
                lhs.source[1][0]*rhs.source[1][0]+
                lhs.source[2][0]*rhs.source[2][0]+
                lhs.source[3][0]*rhs.source[3][0],

                lhs.source[0][0]*rhs.source[0][1]+
                lhs.source[1][0]*rhs.source[1][1]+
                lhs.source[2][0]*rhs.source[2][1]+
                lhs.source[3][0]*rhs.source[3][1],

                lhs.source[0][0]*rhs.source[0][2]+
                lhs.source[1][0]*rhs.source[1][2]+
                lhs.source[2][0]*rhs.source[2][2]+
                lhs.source[3][0]*rhs.source[3][2],

                lhs.source[0][0]*rhs.source[0][3]+
                lhs.source[1][0]*rhs.source[1][3]+
                lhs.source[2][0]*rhs.source[2][3]+
                lhs.source[3][0]*rhs.source[3][3],
            ],
            [
                lhs.source[0][1]*rhs.source[0][0]+
                lhs.source[1][1]*rhs.source[1][0]+
                lhs.source[2][1]*rhs.source[2][0]+
                lhs.source[3][1]*rhs.source[3][0],

                lhs.source[0][1]*rhs.source[0][1]+
                lhs.source[1][1]*rhs.source[1][1]+
                lhs.source[2][1]*rhs.source[2][1]+
                lhs.source[3][1]*rhs.source[3][1],

                lhs.source[0][1]*rhs.source[0][2]+
                lhs.source[1][1]*rhs.source[1][2]+
                lhs.source[2][1]*rhs.source[2][2]+
                lhs.source[3][1]*rhs.source[3][2],

                lhs.source[0][1]*rhs.source[0][3]+
                lhs.source[1][1]*rhs.source[1][3]+
                lhs.source[2][1]*rhs.source[2][3]+
                lhs.source[3][1]*rhs.source[3][3],
            ],
            [
                lhs.source[0][2]*rhs.source[0][0]+
                lhs.source[1][2]*rhs.source[1][0]+
                lhs.source[2][2]*rhs.source[2][0]+
                lhs.source[3][2]*rhs.source[3][0],

                lhs.source[0][2]*rhs.source[0][1]+
                lhs.source[1][2]*rhs.source[1][1]+
                lhs.source[2][2]*rhs.source[2][1]+
                lhs.source[3][2]*rhs.source[3][1],

                lhs.source[0][2]*rhs.source[0][2]+
                lhs.source[1][2]*rhs.source[1][2]+
                lhs.source[2][2]*rhs.source[2][2]+
                lhs.source[3][2]*rhs.source[3][2],

                lhs.source[0][2]*rhs.source[0][3]+
                lhs.source[1][2]*rhs.source[1][3]+
                lhs.source[2][2]*rhs.source[2][3]+
                lhs.source[3][2]*rhs.source[3][3],
            ],
            [
                lhs.source[0][3]*rhs.source[0][0]+
                lhs.source[1][3]*rhs.source[1][0]+
                lhs.source[2][3]*rhs.source[2][0]+
                lhs.source[3][3]*rhs.source[3][0],

                lhs.source[0][3]*rhs.source[0][1]+
                lhs.source[1][3]*rhs.source[1][1]+
                lhs.source[2][3]*rhs.source[2][1]+
                lhs.source[3][3]*rhs.source[3][1],

                lhs.source[0][3]*rhs.source[0][2]+
                lhs.source[1][3]*rhs.source[1][2]+
                lhs.source[2][3]*rhs.source[2][2]+
                lhs.source[3][3]*rhs.source[3][2],

                lhs.source[0][3]*rhs.source[0][3]+
                lhs.source[1][3]*rhs.source[1][3]+
                lhs.source[2][3]*rhs.source[2][3]+
                lhs.source[3][3]*rhs.source[3][3],
            ]
        ]};
    }

    pub fn div(lhs: &mat4, rhs: &mat4) -> mat4 {
        let m2 = mat4::inverse(&rhs);
        return mat4::mul(&lhs, &m2);
    }




    pub fn ortho(top: f32, right: f32, bottom: f32, left: f32, near: f32, far: f32)->mat4{

        return mat4{
            source: [
                [2.0/(right-left), 0.0, 0.0, 0.0],
                [0.0, 2.0/(top-bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0/(far-near), 0.0],
                [-(right+left)/(right-left),-(top+bottom)/(top-bottom),-(far+near)/(far-near),             1.0],
            ]
        };

    }

    pub fn ortho_window(unit: f32, ratio: f32, near: f32, far: f32)->mat4{
        let w = unit * ratio;
        let h = unit;

        return mat4::ortho(h, w, -h, -w, near, far);
    }


    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}


/*

/*
-----------------------------------------------------------------------------
This source file is part of OGRE
    (Object-oriented Graphics Rendering Engine)
For the latest info, see http://www.ogre3d.org/

Copyright (c) 2000-2012 Torus Knot Software Ltd

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
-----------------------------------------------------------------------------
*/
#include "OgreStableHeaders.h"
// NOTE THAT THIS FILE IS BASED ON MATERIAL FROM:

// Geometric Tools, LLC
// Copyright (c) 1998-2010
// Distributed under the Boost Software License, Version 1.0.
// http://www.boost.org/LICENSE_1_0.txt
// http://www.geometrictools.com/License/Boost/LICENSE_1_0.txt

#include "OgreQuaternion.h"

#include "OgreMath.h"
#include "OgreMatrix3.h"
#include "OgreVector3.h"

namespace Ogre {

    const Real Quaternion::msEpsilon = 1e-03;
    const Quaternion Quaternion::ZERO(0,0,0,0);
    const Quaternion Quaternion::IDENTITY(1,0,0,0);

    //-----------------------------------------------------------------------
    void Quaternion::FromRotationMatrix (const Matrix3& kRot)
    {
        // Algorithm in Ken Shoemake's article in 1987 SIGGRAPH course notes
        // article "Quaternion Calculus and Fast Animation".

        Real fTrace = kRot[0][0]+kRot[1][1]+kRot[2][2];
        Real fRoot;

        if ( fTrace > 0.0 )
        {
            // |w| > 1/2, may as well choose w > 1/2
            fRoot = Math::Sqrt(fTrace + 1.0f);  // 2w
            w = 0.5f*fRoot;
            fRoot = 0.5f/fRoot;  // 1/(4w)
            x = (kRot[2][1]-kRot[1][2])*fRoot;
            y = (kRot[0][2]-kRot[2][0])*fRoot;
            z = (kRot[1][0]-kRot[0][1])*fRoot;
        }
        else
        {
            // |w| <= 1/2
            static size_t s_iNext[3] = { 1, 2, 0 };
            size_t i = 0;
            if ( kRot[1][1] > kRot[0][0] )
                i = 1;
            if ( kRot[2][2] > kRot[i][i] )
                i = 2;
            size_t j = s_iNext[i];
            size_t k = s_iNext[j];

            fRoot = Math::Sqrt(kRot[i][i]-kRot[j][j]-kRot[k][k] + 1.0f);
            Real* apkQuat[3] = { &x, &y, &z };
            *apkQuat[i] = 0.5f*fRoot;
            fRoot = 0.5f/fRoot;
            w = (kRot[k][j]-kRot[j][k])*fRoot;
            *apkQuat[j] = (kRot[j][i]+kRot[i][j])*fRoot;
            *apkQuat[k] = (kRot[k][i]+kRot[i][k])*fRoot;
        }
    }
    //-----------------------------------------------------------------------
    void Quaternion::ToRotationMatrix (Matrix3& kRot) const
    {
        Real fTx  = x+x;
        Real fTy  = y+y;
        Real fTz  = z+z;
        Real fTwx = fTx*w;
        Real fTwy = fTy*w;
        Real fTwz = fTz*w;
        Real fTxx = fTx*x;
        Real fTxy = fTy*x;
        Real fTxz = fTz*x;
        Real fTyy = fTy*y;
        Real fTyz = fTz*y;
        Real fTzz = fTz*z;

        kRot[0][0] = 1.0f-(fTyy+fTzz);
        kRot[0][1] = fTxy-fTwz;
        kRot[0][2] = fTxz+fTwy;
        kRot[1][0] = fTxy+fTwz;
        kRot[1][1] = 1.0f-(fTxx+fTzz);
        kRot[1][2] = fTyz-fTwx;
        kRot[2][0] = fTxz-fTwy;
        kRot[2][1] = fTyz+fTwx;
        kRot[2][2] = 1.0f-(fTxx+fTyy);
    }
    //-----------------------------------------------------------------------
    void Quaternion::FromAngleAxis (const Radian& rfAngle,
        const Vector3& rkAxis)
    {
        // assert:  axis[] is unit length
        //
        // The quaternion representing the rotation is
        //   q = cos(A/2)+sin(A/2)*(x*i+y*j+z*k)

        Radian fHalfAngle ( 0.5*rfAngle );
        Real fSin = Math::Sin(fHalfAngle);
        w = Math::Cos(fHalfAngle);
        x = fSin*rkAxis.x;
        y = fSin*rkAxis.y;
        z = fSin*rkAxis.z;
    }
    //-----------------------------------------------------------------------
    void Quaternion::ToAngleAxis (Radian& rfAngle, Vector3& rkAxis) const
    {
        // The quaternion representing the rotation is
        //   q = cos(A/2)+sin(A/2)*(x*i+y*j+z*k)

        Real fSqrLength = x*x+y*y+z*z;
        if ( fSqrLength > 0.0 )
        {
            rfAngle = 2.0*Math::ACos(w);
            Real fInvLength = Math::InvSqrt(fSqrLength);
            rkAxis.x = x*fInvLength;
            rkAxis.y = y*fInvLength;
            rkAxis.z = z*fInvLength;
        }
        else
        {
            // angle is 0 (mod 2*pi), so any axis will do
            rfAngle = Radian(0.0);
            rkAxis.x = 1.0;
            rkAxis.y = 0.0;
            rkAxis.z = 0.0;
        }
    }
    //-----------------------------------------------------------------------
    void Quaternion::FromAxes (const Vector3* akAxis)
    {
        Matrix3 kRot;

        for (size_t iCol = 0; iCol < 3; iCol++)
        {
            kRot[0][iCol] = akAxis[iCol].x;
            kRot[1][iCol] = akAxis[iCol].y;
            kRot[2][iCol] = akAxis[iCol].z;
        }

        FromRotationMatrix(kRot);
    }
    //-----------------------------------------------------------------------
    void Quaternion::FromAxes (const Vector3& xaxis, const Vector3& yaxis, const Vector3& zaxis)
    {
        Matrix3 kRot;

        kRot[0][0] = xaxis.x;
        kRot[1][0] = xaxis.y;
        kRot[2][0] = xaxis.z;

        kRot[0][1] = yaxis.x;
        kRot[1][1] = yaxis.y;
        kRot[2][1] = yaxis.z;

        kRot[0][2] = zaxis.x;
        kRot[1][2] = zaxis.y;
        kRot[2][2] = zaxis.z;

        FromRotationMatrix(kRot);

    }
    //-----------------------------------------------------------------------
    void Quaternion::ToAxes (Vector3* akAxis) const
    {
        Matrix3 kRot;

        ToRotationMatrix(kRot);

        for (size_t iCol = 0; iCol < 3; iCol++)
        {
            akAxis[iCol].x = kRot[0][iCol];
            akAxis[iCol].y = kRot[1][iCol];
            akAxis[iCol].z = kRot[2][iCol];
        }
    }
    //-----------------------------------------------------------------------
    Vector3 Quaternion::xAxis(void) const
    {
        //Real fTx  = 2.0*x;
        Real fTy  = 2.0f*y;
        Real fTz  = 2.0f*z;
        Real fTwy = fTy*w;
        Real fTwz = fTz*w;
        Real fTxy = fTy*x;
        Real fTxz = fTz*x;
        Real fTyy = fTy*y;
        Real fTzz = fTz*z;

        return Vector3(1.0f-(fTyy+fTzz), fTxy+fTwz, fTxz-fTwy);
    }
    //-----------------------------------------------------------------------
    Vector3 Quaternion::yAxis(void) const
    {
        Real fTx  = 2.0f*x;
        Real fTy  = 2.0f*y;
        Real fTz  = 2.0f*z;
        Real fTwx = fTx*w;
        Real fTwz = fTz*w;
        Real fTxx = fTx*x;
        Real fTxy = fTy*x;
        Real fTyz = fTz*y;
        Real fTzz = fTz*z;

        return Vector3(fTxy-fTwz, 1.0f-(fTxx+fTzz), fTyz+fTwx);
    }
    //-----------------------------------------------------------------------
    Vector3 Quaternion::zAxis(void) const
    {
        Real fTx  = 2.0f*x;
        Real fTy  = 2.0f*y;
        Real fTz  = 2.0f*z;
        Real fTwx = fTx*w;
        Real fTwy = fTy*w;
        Real fTxx = fTx*x;
        Real fTxz = fTz*x;
        Real fTyy = fTy*y;
        Real fTyz = fTz*y;

        return Vector3(fTxz+fTwy, fTyz-fTwx, 1.0f-(fTxx+fTyy));
    }
    //-----------------------------------------------------------------------
    void Quaternion::ToAxes (Vector3& xaxis, Vector3& yaxis, Vector3& zaxis) const
    {
        Matrix3 kRot;

        ToRotationMatrix(kRot);

        xaxis.x = kRot[0][0];
        xaxis.y = kRot[1][0];
        xaxis.z = kRot[2][0];

        yaxis.x = kRot[0][1];
        yaxis.y = kRot[1][1];
        yaxis.z = kRot[2][1];

        zaxis.x = kRot[0][2];
        zaxis.y = kRot[1][2];
        zaxis.z = kRot[2][2];
    }

    //-----------------------------------------------------------------------
    Quaternion Quaternion::operator+ (const Quaternion& rkQ) const
    {
        return Quaternion(w+rkQ.w,x+rkQ.x,y+rkQ.y,z+rkQ.z);
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::operator- (const Quaternion& rkQ) const
    {
        return Quaternion(w-rkQ.w,x-rkQ.x,y-rkQ.y,z-rkQ.z);
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::operator* (const Quaternion& rkQ) const
    {
        // NOTE:  Multiplication is not generally commutative, so in most
        // cases p*q != q*p.

        return Quaternion
        (
            w * rkQ.w - x * rkQ.x - y * rkQ.y - z * rkQ.z,
            w * rkQ.x + x * rkQ.w + y * rkQ.z - z * rkQ.y,
            w * rkQ.y + y * rkQ.w + z * rkQ.x - x * rkQ.z,
            w * rkQ.z + z * rkQ.w + x * rkQ.y - y * rkQ.x
        );
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::operator* (Real fScalar) const
    {
        return Quaternion(fScalar*w,fScalar*x,fScalar*y,fScalar*z);
    }
    //-----------------------------------------------------------------------
    Quaternion operator* (Real fScalar, const Quaternion& rkQ)
    {
        return Quaternion(fScalar*rkQ.w,fScalar*rkQ.x,fScalar*rkQ.y,
            fScalar*rkQ.z);
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::operator- () const
    {
        return Quaternion(-w,-x,-y,-z);
    }
    //-----------------------------------------------------------------------
    Real Quaternion::Dot (const Quaternion& rkQ) const
    {
        return w*rkQ.w+x*rkQ.x+y*rkQ.y+z*rkQ.z;
    }
    //-----------------------------------------------------------------------
    Real Quaternion::Norm () const
    {
        return w*w+x*x+y*y+z*z;
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::Inverse () const
    {
        Real fNorm = w*w+x*x+y*y+z*z;
        if ( fNorm > 0.0 )
        {
            Real fInvNorm = 1.0f/fNorm;
            return Quaternion(w*fInvNorm,-x*fInvNorm,-y*fInvNorm,-z*fInvNorm);
        }
        else
        {
            // return an invalid result to flag the error
            return ZERO;
        }
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::UnitInverse () const
    {
        // assert:  'this' is unit length
        return Quaternion(w,-x,-y,-z);
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::Exp () const
    {
        // If q = A*(x*i+y*j+z*k) where (x,y,z) is unit length, then
        // exp(q) = cos(A)+sin(A)*(x*i+y*j+z*k).  If sin(A) is near zero,
        // use exp(q) = cos(A)+A*(x*i+y*j+z*k) since A/sin(A) has limit 1.

        Radian fAngle ( Math::Sqrt(x*x+y*y+z*z) );
        Real fSin = Math::Sin(fAngle);

        Quaternion kResult;
        kResult.w = Math::Cos(fAngle);

        if ( Math::Abs(fSin) >= msEpsilon )
        {
            Real fCoeff = fSin/(fAngle.valueRadians());
            kResult.x = fCoeff*x;
            kResult.y = fCoeff*y;
            kResult.z = fCoeff*z;
        }
        else
        {
            kResult.x = x;
            kResult.y = y;
            kResult.z = z;
        }

        return kResult;
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::Log () const
    {
        // If q = cos(A)+sin(A)*(x*i+y*j+z*k) where (x,y,z) is unit length, then
        // log(q) = A*(x*i+y*j+z*k).  If sin(A) is near zero, use log(q) =
        // sin(A)*(x*i+y*j+z*k) since sin(A)/A has limit 1.

        Quaternion kResult;
        kResult.w = 0.0;

        if ( Math::Abs(w) < 1.0 )
        {
            Radian fAngle ( Math::ACos(w) );
            Real fSin = Math::Sin(fAngle);
            if ( Math::Abs(fSin) >= msEpsilon )
            {
                Real fCoeff = fAngle.valueRadians()/fSin;
                kResult.x = fCoeff*x;
                kResult.y = fCoeff*y;
                kResult.z = fCoeff*z;
                return kResult;
            }
        }

        kResult.x = x;
        kResult.y = y;
        kResult.z = z;

        return kResult;
    }
    //-----------------------------------------------------------------------
    Vector3 Quaternion::operator* (const Vector3& v) const
    {
		// nVidia SDK implementation
		Vector3 uv, uuv;
		Vector3 qvec(x, y, z);
		uv = qvec.crossProduct(v);
		uuv = qvec.crossProduct(uv);
		uv *= (2.0f * w);
		uuv *= 2.0f;

		return v + uv + uuv;

    }
    //-----------------------------------------------------------------------
	bool Quaternion::equals(const Quaternion& rhs, const Radian& tolerance) const
	{
        Real fCos = Dot(rhs);
        Radian angle = Math::ACos(fCos);

		return (Math::Abs(angle.valueRadians()) <= tolerance.valueRadians())
            || Math::RealEqual(angle.valueRadians(), Math::PI, tolerance.valueRadians());


	}
    //-----------------------------------------------------------------------
    Quaternion Quaternion::Slerp (Real fT, const Quaternion& rkP,
        const Quaternion& rkQ, bool shortestPath)
    {
        Real fCos = rkP.Dot(rkQ);
        Quaternion rkT;

        // Do we need to invert rotation?
        if (fCos < 0.0f && shortestPath)
        {
            fCos = -fCos;
            rkT = -rkQ;
        }
        else
        {
            rkT = rkQ;
        }

        if (Math::Abs(fCos) < 1 - msEpsilon)
        {
            // Standard case (slerp)
            Real fSin = Math::Sqrt(1 - Math::Sqr(fCos));
            Radian fAngle = Math::ATan2(fSin, fCos);
            Real fInvSin = 1.0f / fSin;
            Real fCoeff0 = Math::Sin((1.0f - fT) * fAngle) * fInvSin;
            Real fCoeff1 = Math::Sin(fT * fAngle) * fInvSin;
            return fCoeff0 * rkP + fCoeff1 * rkT;
        }
        else
        {
            // There are two situations:
            // 1. "rkP" and "rkQ" are very close (fCos ~= +1), so we can do a linear
            //    interpolation safely.
            // 2. "rkP" and "rkQ" are almost inverse of each other (fCos ~= -1), there
            //    are an infinite number of possibilities interpolation. but we haven't
            //    have method to fix this case, so just use linear interpolation here.
            Quaternion t = (1.0f - fT) * rkP + fT * rkT;
            // taking the complement requires renormalisation
            t.normalise();
            return t;
        }
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::SlerpExtraSpins (Real fT,
        const Quaternion& rkP, const Quaternion& rkQ, int iExtraSpins)
    {
        Real fCos = rkP.Dot(rkQ);
        Radian fAngle ( Math::ACos(fCos) );

        if ( Math::Abs(fAngle.valueRadians()) < msEpsilon )
            return rkP;

        Real fSin = Math::Sin(fAngle);
        Radian fPhase ( Math::PI*iExtraSpins*fT );
        Real fInvSin = 1.0f/fSin;
        Real fCoeff0 = Math::Sin((1.0f-fT)*fAngle - fPhase)*fInvSin;
        Real fCoeff1 = Math::Sin(fT*fAngle + fPhase)*fInvSin;
        return fCoeff0*rkP + fCoeff1*rkQ;
    }
    //-----------------------------------------------------------------------
    void Quaternion::Intermediate (const Quaternion& rkQ0,
        const Quaternion& rkQ1, const Quaternion& rkQ2,
        Quaternion& rkA, Quaternion& rkB)
    {
        // assert:  q0, q1, q2 are unit quaternions

        Quaternion kQ0inv = rkQ0.UnitInverse();
        Quaternion kQ1inv = rkQ1.UnitInverse();
        Quaternion rkP0 = kQ0inv*rkQ1;
        Quaternion rkP1 = kQ1inv*rkQ2;
        Quaternion kArg = 0.25*(rkP0.Log()-rkP1.Log());
        Quaternion kMinusArg = -kArg;

        rkA = rkQ1*kArg.Exp();
        rkB = rkQ1*kMinusArg.Exp();
    }
    //-----------------------------------------------------------------------
    Quaternion Quaternion::Squad (Real fT,
        const Quaternion& rkP, const Quaternion& rkA,
        const Quaternion& rkB, const Quaternion& rkQ, bool shortestPath)
    {
        Real fSlerpT = 2.0f*fT*(1.0f-fT);
        Quaternion kSlerpP = Slerp(fT, rkP, rkQ, shortestPath);
        Quaternion kSlerpQ = Slerp(fT, rkA, rkB);
        return Slerp(fSlerpT, kSlerpP ,kSlerpQ);
    }
    //-----------------------------------------------------------------------
    Real Quaternion::normalise(void)
    {
        Real len = Norm();
        Real factor = 1.0f / Math::Sqrt(len);
        *this = *this * factor;
        return len;
    }
    //-----------------------------------------------------------------------
	Radian Quaternion::getRoll(bool reprojectAxis) const
	{
		if (reprojectAxis)
		{
			// roll = atan2(localx.y, localx.x)
			// pick parts of xAxis() implementation that we need
//			Real fTx  = 2.0*x;
			Real fTy  = 2.0f*y;
			Real fTz  = 2.0f*z;
			Real fTwz = fTz*w;
			Real fTxy = fTy*x;
			Real fTyy = fTy*y;
			Real fTzz = fTz*z;

			// Vector3(1.0-(fTyy+fTzz), fTxy+fTwz, fTxz-fTwy);

			return Radian(Math::ATan2(fTxy+fTwz, 1.0f-(fTyy+fTzz)));

		}
		else
		{
			return Radian(Math::ATan2(2*(x*y + w*z), w*w + x*x - y*y - z*z));
		}
	}
    //-----------------------------------------------------------------------
	Radian Quaternion::getPitch(bool reprojectAxis) const
	{
		if (reprojectAxis)
		{
			// pitch = atan2(localy.z, localy.y)
			// pick parts of yAxis() implementation that we need
			Real fTx  = 2.0f*x;
//			Real fTy  = 2.0f*y;
			Real fTz  = 2.0f*z;
			Real fTwx = fTx*w;
			Real fTxx = fTx*x;
			Real fTyz = fTz*y;
			Real fTzz = fTz*z;

			// Vector3(fTxy-fTwz, 1.0-(fTxx+fTzz), fTyz+fTwx);
			return Radian(Math::ATan2(fTyz+fTwx, 1.0f-(fTxx+fTzz)));
		}
		else
		{
			// internal version
			return Radian(Math::ATan2(2*(y*z + w*x), w*w - x*x - y*y + z*z));
		}
	}
    //-----------------------------------------------------------------------
	Radian Quaternion::getYaw(bool reprojectAxis) const
	{
		if (reprojectAxis)
		{
			// yaw = atan2(localz.x, localz.z)
			// pick parts of zAxis() implementation that we need
			Real fTx  = 2.0f*x;
			Real fTy  = 2.0f*y;
			Real fTz  = 2.0f*z;
			Real fTwy = fTy*w;
			Real fTxx = fTx*x;
			Real fTxz = fTz*x;
			Real fTyy = fTy*y;

			// Vector3(fTxz+fTwy, fTyz-fTwx, 1.0-(fTxx+fTyy));

			return Radian(Math::ATan2(fTxz+fTwy, 1.0f-(fTxx+fTyy)));

		}
		else
		{
			// internal version
			return Radian(Math::ASin(-2*(x*z - w*y)));
		}
	}
    //-----------------------------------------------------------------------
    Quaternion Quaternion::nlerp(Real fT, const Quaternion& rkP,
        const Quaternion& rkQ, bool shortestPath)
    {
		Quaternion result;
        Real fCos = rkP.Dot(rkQ);
		if (fCos < 0.0f && shortestPath)
		{
			result = rkP + fT * ((-rkQ) - rkP);
		}
		else
		{
			result = rkP + fT * (rkQ - rkP);
		}
        result.normalise();
        return result;
    }
}

*/
