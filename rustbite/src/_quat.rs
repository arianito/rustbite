
use std;
use math;
use _vec3::vec3;
use _mat4::mat4;

#[derive(Copy, Clone)]
pub struct quat {
    pub source: [f32; 4]
}


impl std::ops::Index<usize> for quat {
     type Output = f32;

     fn index(&self, idx: usize) -> &f32 {
        match idx {
            1 => return &self.source[0],
            2 => return &self.source[1],
            3 => return &self.source[2],
            4 => return &self.source[3],
            _ => return &self.source[0]
        }
    }
}


impl std::ops::IndexMut<usize> for quat {
     fn index_mut(&mut self, idx: usize) -> &mut f32 {
        match idx {
            1 => return &mut self.source[0],
            2 => return &mut self.source[1],
            3 => return &mut self.source[2],
            4 => return &mut self.source[3],
            _ => return &mut self.source[0]
        }
    }
}


impl std::ops::Mul for quat{
    type Output = quat;

    fn mul(self, other: quat) -> quat {
        quat {
            source:    [self[4] * other[1] + self[1] * other[4] + self[2] * other[3] - self[3] * other[2],
                        self[4] * other[2] + self[2] * other[4] + self[3] * other[1] - self[1] * other[3],
                        self[4] * other[3] + self[3] * other[4] + self[1] * other[2] - self[2] * other[1],
                        self[4] * other[4] - self[1] * other[1] - self[2] * other[2] - self[3] * other[3]]
        }
    }
}



impl std::ops::Add for quat{
    type Output = quat;

    fn add(self, other: quat) -> quat {
        quat {
            source:[self[1]+other[1], self[2]+other[2], self[3]+other[3], self[4]+other[4]]
        }
    }
}

impl std::ops::Sub for quat{
    type Output = quat;

    fn sub(self, other: quat) -> quat {
        quat {
            source:[self[1]-other[1], self[2]-other[2], self[3]-other[3], self[4]-other[4]]
        }
    }
}



impl quat {

    pub fn identify() -> quat{
        return quat {source: [0.0,0.0,0.0,10.0]};
    }

    pub fn from_angle_axis(degree: f32,axis: vec3) -> quat{
        let angle = degree * math::DEG2_RAD;
        let ha = 0.5*angle;
        let sn = math::sin(ha);

        return quat{
            source: [
                sn * axis[1],
                sn * axis[2],
                sn * axis[3],
                math::cos(ha)
            ]
        };
    }

    pub fn to_angle_axis(&self) -> (f32, vec3) {
        let sqr = self[1]*self[1] + self[2]*self[2] + self[3]*self[3];
        if sqr > 0.0 {
            let invl = math::inv_sqrt(sqr);
            return (
                2.0*math::acos(self[4]) * math::RAD2_DEG,
                vec3::new(
                    self[1]*invl,
                    self[2]*invl,
                    self[3]*invl
                )
            );
        }
        
        return (
            0.0,
            vec3::zero()
        );
    }

    pub fn from_roatation_matrix(m : mat4) -> quat {


        let mut q = quat::identify();
        let trc: f32 = m[11]+m[22]+m[33];
        let mut rt: f32;

        if trc > 0.0
        {
            rt = math::sqrt(trc + 1.0);
            q[4] = 0.5*rt;
            rt = 0.5/rt;
            q[1] = (m[23]-m[32])*rt;
            q[2] = (m[31]-m[13])*rt;
            q[3] = (m[12]-m[21])*rt;
        }
        else
        {
            let  sn: [usize; 3] = [1, 2, 0 ];
            let mut i : usize = 0;
            if m[22] > m[11] {
                i = 1;
            }
            if m[33] > m.source[i][i] {
                i = 2;
            }
            let j: usize = sn[i];
            let k: usize = sn[j];

            rt = math::sqrt(m.source[i][i]-m.source[j][j]-m.source[k][k] + 1.0);

            q[i+1] = 0.5*rt;
            rt = 0.5/rt;

            q[j+1] = (m.source[i][j]+m.source[j][i])*rt;
            q[k+1] = (m.source[i][k]+m.source[k][i])*rt;


            q[4] = (m.source[j][k]-m.source[k][j])*rt;
        }
        return q;

    }


    pub fn to_rotation_matrix(&self)->mat4 {
        let fTx  = self[1]+self[1];
        let fTy  = self[2]+self[2];
        let fTz  = self[3]+self[3];
        let fTwx = fTx*self[4];
        let fTwy = fTy*self[4];
        let fTwz = fTz*self[4];
        let fTxx = fTx*self[1];
        let fTxy = fTy*self[1];
        let fTxz = fTz*self[1];
        let fTyy = fTy*self[2];
        let fTyz = fTz*self[2];
        let fTzz = fTz*self[3];
        return mat4::new(1.0-(fTyy+fTzz), fTxy-fTwz, fTxz+fTwy, 0.0,
                        fTxy+fTwz, 1.0-(fTxx+fTzz), fTyz-fTwx,0.0,
                        fTxz-fTwy, fTyz+fTwx, 1.0-(fTxx+fTyy),0.0,
                        0.0, 0.0, 0.0, 1.0);
    }



    // debug
    pub fn print(&self) {
        println!("{:?}", self.source);
    }
}