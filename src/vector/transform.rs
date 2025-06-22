use super::vector3::Vector3;

pub struct Transform {
    /// Rotation around y axis
    pub yaw: f64,
}

impl Transform {
    pub fn new(yaw: f64) -> Self {
        Transform { yaw }
    }

    pub fn vertex_to_world(&self, p: Vector3<f64>) -> Vector3<f64> {
        let (i, j, k) = self.get_basis_vectors();
        Transform::apply_transform(i, j, k, p)
    }

    fn get_basis_vectors(&self) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let i = Vector3::new(f64::cos(self.yaw), 0., f64::sin(self.yaw));
        let j = Vector3::new(0., 1., 0.);
        let k = Vector3::new(-f64::sin(self.yaw), 0., f64::cos(self.yaw));

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
