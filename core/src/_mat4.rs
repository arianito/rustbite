use std;
use _vec3::vec3;
use _quat::quat;


pub struct mat4 {
    pub source: [[f32; 4]; 4]
}

impl std::ops::Mul for mat4{
    type Output = mat4;

    fn mul(self, other: mat4) -> mat4 {
        mat4::mul_mat(&self, &other)
    }
}

impl std::ops::Mul<vec3> for mat4{
    type Output = vec3;
    fn mul(self, other: vec3) -> vec3 {
        mat4::mul_vec(&self, &other)
    }
}


impl std::ops::Index<usize> for mat4 {
     type Output = f32;

     fn index(&self, idx: usize) -> &f32 {
        match idx {
            11 => return &self.source[0][0],
            12 => return &self.source[1][0],
            13 => return &self.source[2][0],
            14 => return &self.source[3][0],

            21 => return &self.source[0][1],
            22 => return &self.source[1][1],
            23 => return &self.source[2][1],
            24 => return &self.source[3][1],


            31 => return &self.source[0][2],
            32 => return &self.source[1][2],
            33 => return &self.source[2][2],
            34 => return &self.source[3][2],

            41 => return &self.source[0][3],
            42 => return &self.source[1][3],
            43 => return &self.source[2][3],
            44 => return &self.source[3][3],
            _ => return &self.source[0][0]
        }
    }
}


impl std::ops::IndexMut<usize> for mat4 {

     fn index_mut(&mut self, idx: usize) -> &mut f32 {
        match idx {
            11 => return &mut self.source[0][0],
            12 => return &mut self.source[1][0],
            13 => return &mut self.source[2][0],
            14 => return &mut self.source[3][0],

            21 => return &mut self.source[0][1],
            22 => return &mut self.source[1][1],
            23 => return &mut self.source[2][1],
            24 => return &mut self.source[3][1],


            31 => return &mut self.source[0][2],
            32 => return &mut self.source[1][2],
            33 => return &mut self.source[2][2],
            34 => return &mut self.source[3][2],

            41 => return &mut self.source[0][3],
            42 => return &mut self.source[1][3],
            43 => return &mut self.source[2][3],
            44 => return &mut self.source[3][3],

            _ => return &mut self.source[0][0],
        }
    }
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
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
     m21: f32, m22: f32, m23: f32, m24: f32,
     m31: f32,m32: f32,m33: f32, m34: f32,
     m41: f32,m42: f32,m43: f32, m44: f32)->mat4{
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
                [a[1], a[2], a[3], 1.0f32]
            ]
        };
    }
    pub fn create_rotation(q: &quat)->mat4{
        return q.to_rotation_matrix();
    }
    pub fn create_scale(a: &vec3)->mat4{
        return mat4{
            source: [
                [a[1], 0.0f32  , 0.0f32, 0.0f32],
                [0.0f32    , a[2], 0.0f32, 0.0f32],
                [0.0f32    , 0.0f32  , a[3], 0.0f32],
                [0.0f32    , 0.0f32  , 0.0f32, 1.0f32]
            ]
        };
    }

    pub fn create_trs(position: &vec3, quaternion: &quat, scale: &vec3)->mat4{
        return mat4::create_scale(&scale) * mat4::create_rotation(&quaternion) * mat4::create_translation(&position);
    }


    pub fn determinant(lhs: &mat4) -> f32 {
        return  lhs[11] * lhs[22] * lhs[33] * lhs[44] -
                lhs[11] * lhs[22] * lhs[34] * lhs[43] -
                lhs[11] * lhs[23] * lhs[32] * lhs[44] +
                lhs[11] * lhs[23] * lhs[34] * lhs[42] +
                lhs[11] * lhs[24] * lhs[32] * lhs[43] -
                lhs[11] * lhs[24] * lhs[33] * lhs[42] -
                lhs[12] * lhs[21] * lhs[33] * lhs[44] +
                lhs[12] * lhs[21] * lhs[34] * lhs[43] +
                lhs[12] * lhs[23] * lhs[31] * lhs[44] -
                lhs[12] * lhs[23] * lhs[34] * lhs[41] -
                lhs[12] * lhs[24] * lhs[31] * lhs[43] +
                lhs[12] * lhs[24] * lhs[33] * lhs[41] +
                lhs[13] * lhs[21] * lhs[32] * lhs[44] -
                lhs[13] * lhs[21] * lhs[34] * lhs[42] -
                lhs[13] * lhs[22] * lhs[31] * lhs[44] +
                lhs[13] * lhs[22] * lhs[34] * lhs[41] +
                lhs[13] * lhs[24] * lhs[31] * lhs[42] -
                lhs[13] * lhs[24] * lhs[32] * lhs[41] -
                lhs[14] * lhs[21] * lhs[32] * lhs[43] +
                lhs[14] * lhs[21] * lhs[33] * lhs[42] +
                lhs[14] * lhs[22] * lhs[31] * lhs[43] -
                lhs[14] * lhs[22] * lhs[33] * lhs[41] -
                lhs[14] * lhs[23] * lhs[31] * lhs[42] +
                lhs[14] * lhs[23] * lhs[32] * lhs[41];
    }

    pub fn transpose(lhs: &mat4) -> mat4 {
        return mat4::new(
            lhs[11],lhs[21],lhs[31],lhs[41],
            lhs[12],lhs[22],lhs[32],lhs[42],
            lhs[13],lhs[23],lhs[33],lhs[43],
            lhs[14],lhs[24],lhs[34],lhs[44],
        );
    }

    pub fn inverse(lhs: &mat4) -> mat4 {
        let det = mat4::determinant(&lhs);
        if det == 0.0 {
            return mat4::identify(1.0);
        } else {
        return mat4::new(
                (lhs[22] * lhs[33] * lhs[44] + lhs[23] * lhs[34] * lhs[42] +
                lhs[24] * lhs[32] * lhs[43] - lhs[22] * lhs[34] * lhs[43] -
                lhs[23] * lhs[32] * lhs[44] - lhs[24] * lhs[33] * lhs[42])/det,

                (lhs[12] * lhs[34] * lhs[43] + lhs[13] * lhs[32] * lhs[44] +
                lhs[14] * lhs[33] * lhs[42] - lhs[12] * lhs[33] * lhs[44] -
                lhs[13] * lhs[34] * lhs[42] - lhs[14] * lhs[32] * lhs[43])/det,

                (lhs[12] * lhs[23] * lhs[44] + lhs[13] * lhs[24] * lhs[42] +
                lhs[14] * lhs[22] * lhs[4314] - lhs[12] * lhs[24] * lhs[43] -
                lhs[13] * lhs[22] * lhs[44] - lhs[14] * lhs[23] * lhs[42])/det,

                (lhs[12] * lhs[24] * lhs[33] + lhs[13] * lhs[22] * lhs[34] +
                lhs[14] * lhs[23] * lhs[32] - lhs[12] * lhs[23] * lhs[34] -
                lhs[13] * lhs[24] * lhs[32] - lhs[14] * lhs[22] * lhs[33])/det,

                (lhs[21] * lhs[34] * lhs[43] + lhs[23] * lhs[31] * lhs[44] +
                lhs[24] * lhs[33] * lhs[41] - lhs[21] * lhs[33] * lhs[44] -
                lhs[23] * lhs[34] * lhs[41] - lhs[24] * lhs[31] * lhs[43])/det,

                (lhs[11] * lhs[33] * lhs[44] + lhs[13] * lhs[34] * lhs[41] +
                lhs[14] * lhs[31] * lhs[43] - lhs[11] * lhs[34] * lhs[43] -
                lhs[13] * lhs[31] * lhs[44] - lhs[14] * lhs[33] * lhs[41])/det,

                (lhs[11] * lhs[24] * lhs[43] + lhs[13] * lhs[21] * lhs[44] +
                lhs[14] * lhs[23] * lhs[41] - lhs[11] * lhs[23] * lhs[44] -
                lhs[13] * lhs[24] * lhs[41] - lhs[14] * lhs[21] * lhs[43])/det,

                (lhs[11] * lhs[23] * lhs[34] + lhs[13] * lhs[24] * lhs[31] +
                lhs[14] * lhs[21] * lhs[33] - lhs[11] * lhs[24] * lhs[33] -
                lhs[13] * lhs[21] * lhs[34] - lhs[14] * lhs[23] * lhs[31])/det,

                (lhs[21] * lhs[32] * lhs[44] + lhs[22] * lhs[34] * lhs[41] +
                lhs[24] * lhs[31] * lhs[42] - lhs[21] * lhs[34] * lhs[42] -
                lhs[22] * lhs[31] * lhs[44] - lhs[24] * lhs[32] * lhs[41])/det,

                (lhs[11] * lhs[34] * lhs[42] + lhs[12] * lhs[31] * lhs[44] +
                lhs[14] * lhs[32] * lhs[41] - lhs[11] * lhs[32] * lhs[44] -
                lhs[12] * lhs[34] * lhs[41] - lhs[14] * lhs[31] * lhs[42])/det,

                (lhs[11] * lhs[22] * lhs[44] + lhs[12] * lhs[24] * lhs[41] +
                lhs[14] * lhs[21] * lhs[42] - lhs[11] * lhs[24] * lhs[42] -
                lhs[12] * lhs[21] * lhs[44] - lhs[14] * lhs[22] * lhs[41])/det,

                (lhs[11] * lhs[24] * lhs[32] + lhs[12] * lhs[21] * lhs[34] +
                lhs[14] * lhs[22] * lhs[31] - lhs[11] * lhs[22] * lhs[34] -
                lhs[12] * lhs[24] * lhs[31] - lhs[14] * lhs[21] * lhs[32])/det,

                (lhs[21] * lhs[33] * lhs[42] + lhs[22] * lhs[31] * lhs[43] +
                lhs[23] * lhs[32] * lhs[41] - lhs[21] * lhs[32] * lhs[43] -
                lhs[22] * lhs[33] * lhs[41] - lhs[23] * lhs[31] * lhs[42])/det,

                (lhs[11] * lhs[32] * lhs[43] + lhs[12] * lhs[33] * lhs[41] +
                lhs[13] * lhs[31] * lhs[42] - lhs[11] * lhs[33] * lhs[42] -
                lhs[12] * lhs[31] * lhs[43] - lhs[13] * lhs[32] * lhs[41])/det,

                (lhs[11] * lhs[23] * lhs[42] + lhs[12] * lhs[21] * lhs[43] +
                lhs[13] * lhs[22] * lhs[41] - lhs[11] * lhs[22] * lhs[43] -
                lhs[12] * lhs[23] * lhs[41] - lhs[13] * lhs[21] * lhs[42])/det,

                (lhs[11] * lhs[22] * lhs[33] + lhs[12] * lhs[23] * lhs[31] +
                lhs[13] * lhs[21] * lhs[32] - lhs[11] * lhs[23] * lhs[32] -
                lhs[12] * lhs[21] * lhs[33] - lhs[13] * lhs[22] * lhs[31])/det
            );
        }
    }


    pub fn mul_mat(lhs: &mat4, rhs: &mat4) -> mat4 {
        return mat4::new(
                lhs[11]*rhs[11]+
                lhs[12]*rhs[21]+
                lhs[13]*rhs[31]+
                lhs[14]*rhs[41],

                lhs[11]*rhs[12]+
                lhs[12]*rhs[22]+
                lhs[13]*rhs[32]+
                lhs[14]*rhs[42],

                lhs[11]*rhs[13]+
                lhs[12]*rhs[23]+
                lhs[13]*rhs[33]+
                lhs[14]*rhs[43],

                lhs[11]*rhs[14]+
                lhs[12]*rhs[24]+
                lhs[13]*rhs[34]+
                lhs[14]*rhs[44],

                lhs[21]*rhs[11]+
                lhs[22]*rhs[21]+
                lhs[23]*rhs[31]+
                lhs[24]*rhs[41],

                lhs[21]*rhs[12]+
                lhs[22]*rhs[22]+
                lhs[23]*rhs[32]+
                lhs[24]*rhs[42],

                lhs[21]*rhs[13]+
                lhs[22]*rhs[23]+
                lhs[23]*rhs[33]+
                lhs[24]*rhs[43],

                lhs[21]*rhs[14]+
                lhs[22]*rhs[24]+
                lhs[23]*rhs[34]+
                lhs[24]*rhs[44],

                lhs[31]*rhs[11]+
                lhs[32]*rhs[21]+
                lhs[33]*rhs[31]+
                lhs[34]*rhs[41],

                lhs[31]*rhs[12]+
                lhs[32]*rhs[22]+
                lhs[33]*rhs[32]+
                lhs[34]*rhs[42],

                lhs[31]*rhs[13]+
                lhs[32]*rhs[23]+
                lhs[33]*rhs[33]+
                lhs[34]*rhs[43],

                lhs[31]*rhs[14]+
                lhs[32]*rhs[24]+
                lhs[33]*rhs[34]+
                lhs[34]*rhs[44],

                lhs[41]*rhs[11]+
                lhs[42]*rhs[21]+
                lhs[43]*rhs[31]+
                lhs[44]*rhs[41],

                lhs[41]*rhs[12]+
                lhs[42]*rhs[22]+
                lhs[43]*rhs[32]+
                lhs[44]*rhs[42],

                lhs[41]*rhs[13]+
                lhs[42]*rhs[23]+
                lhs[43]*rhs[33]+
                lhs[44]*rhs[43],

                lhs[41]*rhs[14]+
                lhs[42]*rhs[24]+
                lhs[43]*rhs[34]+
                lhs[44]*rhs[44] );
    }


    pub fn mul_vec(lhs: &mat4, rhs: &vec3) -> vec3 {
        return vec3::new(
                lhs[11]*rhs[1]+
                lhs[12]*rhs[2]+
                lhs[13]*rhs[3]+
                lhs[14],

                lhs[21]*rhs[1]+
                lhs[22]*rhs[2]+
                lhs[23]*rhs[3]+
                lhs[24],

                lhs[31]*rhs[1]+
                lhs[32]*rhs[2]+
                lhs[33]*rhs[3]+
                lhs[34]);
    }

    pub fn div(lhs: &mat4, rhs: &mat4) -> mat4 {
        let m2 = mat4::inverse(&rhs);
        return mat4::mul_mat(&lhs, &m2);
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
    pub fn clone(&self) -> mat4{
        return mat4{
            source: [
                [self.source[0][0], self.source[0][1],self.source[0][2],self.source[0][3]],
                [self.source[1][0], self.source[1][1],self.source[1][2],self.source[1][3]],
                [self.source[2][0], self.source[2][1],self.source[2][2],self.source[2][3]],
                [self.source[3][0], self.source[3][1],self.source[3][2],self.source[3][3]],
            ]
        };
    }
    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}