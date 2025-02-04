use glam::{Mat4, Vec3, Quat, Mat3, Vec4};

pub struct Camera {
    pub fov: f32,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub rotation: Quat
}


impl Camera {

    pub fn init(position: Vec3, fov: f32) -> Self {
        let rotation = Quat::IDENTITY;
        let front = rotation.mul_vec3(Vec3::NEG_Z);
        let right = rotation.mul_vec3(Vec3::X);
        let up = rotation.mul_vec3(Vec3::Y);

        Self { position, fov, rotation, front, right, up }
    }

    fn update_vectors(&mut self) {
        self.front = self.rotation.mul_vec3(Vec3::NEG_Z);
        self.right = self.rotation.mul_vec3(Vec3::X);
        self.front = self.rotation.mul_vec3(Vec3::Y);
    }

    pub fn rotate(&mut self, x: f32, y:f32, z:f32) {
        self.rotation *= Quat::from_rotation_z(z) * Quat::from_rotation_y(y) * Quat::from_rotation_x(x);
        self.update_vectors();
    }

    pub fn get_projection(&self, width: f32, height: f32) -> Mat4 {
        let aspect = width / height;
        Mat4::perspective_rh(self.fov, aspect, 0.1, 100.0)
    }

    pub fn get_view(&self) -> Mat4{
        Mat4::look_at_rh(self.position, self.position + self.front, self.up) // ????
    }
}