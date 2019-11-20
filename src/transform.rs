use crate::*;

pub struct Transform {
    scale: Vec3,
    translation: Vec3,
    rotation: Quat,
}

impl From<Transform> for Mat4 {
    fn from(t: Transform) -> Self {
        let scale_mat = Mat4::from_nonuniform_scale(t.scale.x, t.scale.y, t.scale.z);
        let rot_mat = Mat4::from(t.rotation);
        let trans_mat = Mat4::from_translation(t.translation);

        trans_mat * rot_mat * scale_mat
    }
}
