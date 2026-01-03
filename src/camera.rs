use nalgebra_glm as glm;

use glm::Vec3;

pub struct Camera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    front: Vec3,
    up: Vec3,
    right: Vec3
}

impl Camera {
    pub fn new(position: Vec3, yaw: f32, pitch: f32) -> Camera {
        let mut cam = Camera {
            position,
            yaw,
            pitch,
            front: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),
            right: glm::vec3(0., 0., 0.),
        };

        cam.update();
        cam
    }

    fn update(&mut self) {
        self.front.x = self.yaw.cos() * self.pitch.cos();
        self.front.y = self.pitch.sin();
        self.front.z = self.yaw.sin() * self.pitch.cos();

        self.front = glm::normalize(&self.front);
        self.right = glm::normalize(&glm::cross(&self.front, &self.up));
    }

    pub fn make_view(&self) -> glm::Mat4 {
        glm::look_at(
            &self.position,
            &(self.position + self.front),
            &self.up
        )
    }

    pub fn move_front(&mut self, amount: f32) {
        self.position += amount * self.front;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.position += amount * self.right;
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;
        
        if self.pitch > 1.553 {self.pitch = 1.553;}
        if self.pitch < -1.553 {self.pitch = -1.553;}

        self.update();
    }
}
