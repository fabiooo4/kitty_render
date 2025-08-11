use super::vector3::Vector3;

#[derive(Default)]
pub struct Transform {
    /// Rotation around y axis
    pub yaw: f64,
    /// Rotation around x axis
    pub pitch: f64,
    /// Position in world space
    pub position: Vector3<f64>,
}

impl Transform {
    pub fn new(yaw: f64, pitch: f64) -> Self {
        Transform {
            yaw,
            pitch,
            position: Vector3::default(),
        }
    }

    pub fn vertex_to_world(&self, p: Vector3<f64>) -> Vector3<f64> {
        let (i, j, k) = self.get_basis_vectors();
        Transform::apply_transform(i, j, k, p) + self.position
    }

    fn get_basis_vectors(&self) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let i_yaw = Vector3::new(f64::cos(self.yaw), 0., f64::sin(self.yaw));
        let j_yaw = Vector3::new(0., 1., 0.);
        let k_yaw = Vector3::new(-f64::sin(self.yaw), 0., f64::cos(self.yaw));

        let i_pitch = Vector3::new(1., 0., 0.);
        let j_pitch = Vector3::new(0., f64::cos(self.pitch), -f64::sin(self.pitch));
        let k_pitch = Vector3::new(0., f64::sin(self.pitch), f64::cos(self.pitch));

        let i = Transform::apply_transform(i_yaw, j_yaw, k_yaw, i_pitch);
        let j = Transform::apply_transform(i_yaw, j_yaw, k_yaw, j_pitch);
        let k = Transform::apply_transform(i_yaw, j_yaw, k_yaw, k_pitch);

        (i, j, k)
    }

    fn apply_transform(
        i: Vector3<f64>,
        j: Vector3<f64>,
        k: Vector3<f64>,
        p: Vector3<f64>,
    ) -> Vector3<f64> {
        i * p.x + j * p.y + k * p.z
    }
}
