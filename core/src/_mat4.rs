use std;
use _vec3::vec3;
use _quat::quat;

pub struct mat4 {
    pub source: [[f32; 4]; 4]
}

impl std::ops::Mul for mat4{
    type Output = mat4;

    fn mul(self, other: mat4) -> mat4 {
        mat4::mul(&self, &other)
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
        return mat4::create_scale(&scale) * mat4::create_rotation(&quaternion) * mat4::create_translation(&position);
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