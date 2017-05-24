use std;
use vec3;
use quat;


#[derive(Copy, Clone)]
pub struct mat4 {
    pub source: [[f32; 4]; 4]
}

impl std::ops::Mul for mat4{
    type Output = mat4;

    fn mul(self, other: mat4) -> mat4 {
        return mat4::new(
                self[11]*other[11]+
                self[12]*other[21]+
                self[13]*other[31]+
                self[14]*other[41],

                self[11]*other[12]+
                self[12]*other[22]+
                self[13]*other[32]+
                self[14]*other[42],

                self[11]*other[13]+
                self[12]*other[23]+
                self[13]*other[33]+
                self[14]*other[43],

                self[11]*other[14]+
                self[12]*other[24]+
                self[13]*other[34]+
                self[14]*other[44],

                self[21]*other[11]+
                self[22]*other[21]+
                self[23]*other[31]+
                self[24]*other[41],

                self[21]*other[12]+
                self[22]*other[22]+
                self[23]*other[32]+
                self[24]*other[42],

                self[21]*other[13]+
                self[22]*other[23]+
                self[23]*other[33]+
                self[24]*other[43],

                self[21]*other[14]+
                self[22]*other[24]+
                self[23]*other[34]+
                self[24]*other[44],

                self[31]*other[11]+
                self[32]*other[21]+
                self[33]*other[31]+
                self[34]*other[41],

                self[31]*other[12]+
                self[32]*other[22]+
                self[33]*other[32]+
                self[34]*other[42],

                self[31]*other[13]+
                self[32]*other[23]+
                self[33]*other[33]+
                self[34]*other[43],

                self[31]*other[14]+
                self[32]*other[24]+
                self[33]*other[34]+
                self[34]*other[44],

                self[41]*other[11]+
                self[42]*other[21]+
                self[43]*other[31]+
                self[44]*other[41],

                self[41]*other[12]+
                self[42]*other[22]+
                self[43]*other[32]+
                self[44]*other[42],

                self[41]*other[13]+
                self[42]*other[23]+
                self[43]*other[33]+
                self[44]*other[43],

                self[41]*other[14]+
                self[42]*other[24]+
                self[43]*other[34]+
                self[44]*other[44] );
    }
}

impl std::ops::Mul<vec3> for mat4{
    type Output = vec3;
    fn mul(self, other: vec3) -> vec3 {
        
        return vec3::new(
                self[11]*other[1]+
                self[12]*other[2]+
                self[13]*other[3]+
                self[14],

                self[21]*other[1]+
                self[22]*other[2]+
                self[23]*other[3]+
                self[24],

                self[31]*other[1]+
                self[32]*other[2]+
                self[33]*other[3]+
                self[34]);

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
    pub fn create_translation(a: vec3)->mat4{
        return mat4{
            source: [
                [1.0f32, 0.0f32, 0.0f32, 0.0],
                [0.0f32, 1.0f32, 0.0f32, 0.0],
                [0.0f32, 0.0f32, 1.0f32, 0.0],
                [a[1], a[2], a[3], 1.0f32]
            ]
        };
    }
    pub fn create_rotation(q: quat)->mat4{
        return q.to_rotation_matrix();
    }
    pub fn create_scale(a: vec3)->mat4{
        return mat4{
            source: [
                [a[1], 0.0f32  , 0.0f32, 0.0f32],
                [0.0f32    , a[2], 0.0f32, 0.0f32],
                [0.0f32    , 0.0f32  , a[3], 0.0f32],
                [0.0f32    , 0.0f32  , 0.0f32, 1.0f32]
            ]
        };
    }

    

    pub fn create_trs(position: vec3, quaternion: quat, scale: vec3)->mat4{
        return  mat4::create_translation(position) * mat4::create_scale(scale) * mat4::create_rotation(quaternion);
    }
    
    pub fn determinant(lhs: mat4) -> f32 {
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

    pub fn transpose(lhs: mat4) -> mat4 {
        return mat4::new(
            lhs[11],lhs[21],lhs[31],lhs[41],
            lhs[12],lhs[22],lhs[32],lhs[42],
            lhs[13],lhs[23],lhs[33],lhs[43],
            lhs[14],lhs[24],lhs[34],lhs[44],
        );
    }

    pub fn inverse(lhs: mat4) -> mat4 {
        let det = mat4::determinant(lhs);
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

    pub fn div(lhs: mat4, rhs: mat4) -> mat4 {
        return lhs * mat4::inverse(rhs);
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
        let w = unit;
        let h = unit / ratio;

        return mat4::ortho(h, w, -h, -w, near, far);
    }
    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}