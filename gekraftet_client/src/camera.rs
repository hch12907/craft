use cgmath::{ Deg, InnerSpace, Matrix4, Point3, Rad, Vector3 };

pub struct Camera {
    position: Point3<f32>,
    target: Vector3<f32>,
    sensitivity: f32,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    pub fn new(position: Point3<f32>, target: Vector3<f32>) -> Self {
        Self {
            position,
            target,
            sensitivity: 0.325,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn front(&self) -> Vector3<f32> {
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

        let deg90 = Rad::from(Deg(89.9f32));
        let deg90 = deg90.0;

        // Prevent camera from flipping y
        self.pitch = if self.pitch >= deg90 {
            deg90
        } else if self.pitch <= -deg90 {
            -deg90
        } else {
            self.pitch
        };

        self.target = Vector3::<f32>::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        ).normalize();
    }

    pub fn move_camera(&mut self, pos: Point3<f32>) {
        self.position = pos;
    }

    pub fn generate_view(&self) -> Matrix4<f32> {
        Matrix4::<f32>::look_at(
            self.position, 
            self.position + self.target, 
            Vector3::<f32>::new(0.0, 1.0, 0.0)
        )
    }
}
