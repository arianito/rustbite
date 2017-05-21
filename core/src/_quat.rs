
use DEG2_RAD;
use _vec3::vec3;
use _mat4::mat4;

pub struct quat {
    pub source: [f32; 4]
}

impl quat {

    pub fn identify() -> quat{
        return quat {source: [0.0,0.0,0.0,10.0]};
    }

    pub fn from_angle_axis(degree: f32,axis: &vec3) -> quat{
        let angle = degree * self::DEG2_RAD;
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