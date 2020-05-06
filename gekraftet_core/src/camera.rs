use crate::maths::*;

pub struct Camera {
    position: Vector3F,
    target: Vector3F,
    sensitivity: f32,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    pub fn new(position: Vector3F, target: Vector3F) -> Self {
        Self {
            position,
            target,
            sensitivity: 0.325,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn front(&self) -> Vector3F {
        self.target
    }

    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn set_sensitivity(&mut self, s: f32) {
        self.sensitivity = s;
    }

    pub fn rotate_by_mouse(&mut self, x: f32, y: f32, delta_time: f32) {
        let x_angle =  delta_time * x * self.sensitivity;
        let y_angle = -delta_time * y * self.sensitivity;
        
        self.yaw += x_angle;
        self.pitch += y_angle;

        let deg90: Rad = Deg(89.9).into();
        let deg90 = deg90.0;

        // Prevent camera from flipping y
        self.pitch = if self.pitch >= deg90 {
            deg90
        } else if self.pitch <= -deg90 {
            -deg90
        } else {
            self.pitch
        };

        self.target = Vector3F::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        ).normalize();
    }

    pub fn move_camera(&mut self, pos: Vector3F) {
        self.position = pos;
    }

    pub fn generate_view(&self) -> Matrix4 {
        Matrix4::look_at(
            self.position, 
            self.position + self.target, 
            Vector3F::new(0.0, 1.0, 0.0)
        )
    }
}
