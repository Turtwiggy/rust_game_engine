#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use cgmath;
use cgmath::vec3;
use cgmath::prelude::*;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN
}
use self::CameraMovement::*;

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    // Camera Attributes
    pub Position: Point3,
    pub Front: Vector3,
    pub Up: Vector3,
    pub Right: Vector3,
    pub WorldUp: Vector3,
    // Euler Angles
    pub Yaw: f32,
    pub Pitch: f32,
    // Camera options
    pub MovementSpeed: f32,
    pub MouseSensitivity: f32,
    pub Zoom: f32,
    pub state : CameraState,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraState {
    fwd_pressed : bool,
    bwd_pressed : bool,
    lft_pressed : bool,
    rgt_pressed : bool,
    up_pressed : bool,
    dwn_pressed : bool,
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            Position: Point3::new(0.0, 0.0, 0.0),
            Front: vec3(0.0, 0.0, -1.0),
            Up: Vector3::zero(), // initialized later
            Right: Vector3::zero(), // initialized later
            WorldUp: Vector3::unit_y(),
            Yaw: YAW,
            Pitch: PITCH,
            MovementSpeed: SPEED,
            MouseSensitivity: SENSITIVTY,
            Zoom: ZOOM,
            state: CameraState {
                fwd_pressed: false, 
                bwd_pressed: false, 
                lft_pressed: false, 
                rgt_pressed: false,
                up_pressed: false,
                dwn_pressed: false,
            }
        };

        // let mut firstMouse = true;
        // let mut lastX: f32 = init_width as f32 / 2.0;
        // let mut lastY: f32 = init_height as f32 / 2.0;

        camera.updateCameraVectors();
        camera
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        Matrix4::look_at(self.Position, self.Position + self.Front, self.Up)
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn ProcessKeyboardDown(&mut self, direction: CameraMovement) {
        if direction == FORWARD {
            self.state.fwd_pressed = true;
        }
        if direction == BACKWARD {
            self.state.bwd_pressed = true;
        }
        if direction == LEFT {
            self.state.lft_pressed = true;
        }
        if direction == RIGHT {
            self.state.rgt_pressed = true;
        }
        if direction == UP {
            self.state.up_pressed = true;
        }
        if direction == DOWN {
            self.state.dwn_pressed = true;
        }
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn ProcessKeyboardUp(&mut self, direction: CameraMovement) {
        if direction == FORWARD {
            self.state.fwd_pressed = false;
        }
        if direction == BACKWARD {
            self.state.bwd_pressed = false;
        }
        if direction == LEFT {
            self.state.lft_pressed = false;
        }
        if direction == RIGHT {
            self.state.rgt_pressed = false;
        }
        if direction == UP {
            self.state.up_pressed = false;
        }
        if direction == DOWN {
            self.state.dwn_pressed = false;
        }
    }

    pub fn Update(&mut self, delta_time : f64) 
    {
        let state = self.state;
        let velocity = self.MovementSpeed * delta_time as f32;

        if state.fwd_pressed {
            self.Position += self.Front * velocity;
        }
        else if state.bwd_pressed {
            self.Position += -(self.Front * velocity);
        }

        if state.lft_pressed {
            self.Position += -(self.Right * velocity);
        }
        else if state.rgt_pressed {
            self.Position += self.Right * velocity;
        }

        if state.up_pressed {
            //self.Position += -(self.Right * velocity);
            self.Position += self.Up * velocity;
        }
        else if state.dwn_pressed {
            //self.Position += self.Right * velocity;
            self.Position += -(self.Up * velocity);
        }
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn ProcessMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, constrainPitch: bool) {
        xoffset *= self.MouseSensitivity;
        yoffset *= self.MouseSensitivity;

        self.Yaw += xoffset;
        self.Pitch += yoffset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrainPitch {
            if self.Pitch > 89.0 {
                self.Pitch = 89.0;
            }
            if self.Pitch < -89.0 {
                self.Pitch = -89.0;
            }
        }

        // Update Front, Right and Up Vectors using the updated Eular angles
        self.updateCameraVectors();
    }

    // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn ProcessMouseScroll(&mut self, yoffset: f32) {
        if self.Zoom >= 1.0 && self.Zoom <= 45.0 {
            self.Zoom -= yoffset;
        }
        if self.Zoom <= 1.0 {
            self.Zoom = 1.0;
        }
        if self.Zoom >= 45.0 {
            self.Zoom = 45.0;
        }
    }

    /// Calculates the front vector from the Camera's (updated) Eular Angles
    fn updateCameraVectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos(),
            y: self.Pitch.to_radians().sin(),
            z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos(),
        };
        self.Front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.Right = self.Front.cross(self.WorldUp).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.Up = self.Right.cross(self.Front).normalize();
    }
}