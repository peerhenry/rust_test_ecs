use gl::types::*;
use cgmath::{ Matrix4, Ortho, Point3, Vector3 };

pub struct OrthoCamera {
  // view
  pub eye: Point3<GLfloat>,
  pub target: Point3<GLfloat>,
  pub up: Vector3<GLfloat>,
  pub view_matrix: Matrix4<GLfloat>,
  // ortho
  pub aspect: GLfloat,
  pub height: GLfloat,
  pub near:   GLfloat,
  pub far:    GLfloat,
  pub projection_matrix: Matrix4<GLfloat>,
}

pub struct OrthoCameraBuilder {
  // view
  pub eye: Point3<GLfloat>,
  pub target: Point3<GLfloat>,
  pub up: Vector3<GLfloat>,
  // ortho
  pub aspect: GLfloat,
  pub height: GLfloat,
  pub near:   GLfloat,
  pub far:    GLfloat,
}

impl OrthoCameraBuilder {
  #[allow(dead_code)]
  pub fn new() -> OrthoCameraBuilder {
    OrthoCameraBuilder {
      eye: Point3::new(0.0, 0.0, -2.0),
      target: Point3::new(0.0, 0.0, 0.0),
      up: Vector3::new(0.0, 1.0, 0.0),

      aspect: 16.0/9.0,
      height: 1.0,

      near: 0.1,
      far: 100.0,
    }
  }

  #[allow(dead_code)]
  pub fn with_eye(mut self, eye: Point3<GLfloat>) -> OrthoCameraBuilder {
    self.eye = eye;
    self
  }

  #[allow(dead_code)]
  pub fn with_target(mut self, target: Point3<GLfloat>) -> OrthoCameraBuilder {
    self.target = target;
    self
  }

  #[allow(dead_code)]
  pub fn with_up(mut self, up: Vector3<GLfloat>) -> OrthoCameraBuilder {
    self.up = up;
    self
  }

  #[allow(dead_code)]
  pub fn with_aspect(mut self, aspect: GLfloat) -> OrthoCameraBuilder {
    self.aspect = aspect;
    self
  }

  #[allow(dead_code)]
  pub fn with_height(mut self, height: GLfloat) -> OrthoCameraBuilder {
    self.height = height;
    self
  }

  #[allow(dead_code)]
  pub fn with_near(mut self, near: GLfloat) -> OrthoCameraBuilder {
    self.near = near;
    self
  }

  #[allow(dead_code)]
  pub fn with_far(mut self, far: GLfloat) -> OrthoCameraBuilder {
    self.far = far;
    self
  }

  #[allow(dead_code)]
  pub fn build(self) -> OrthoCamera {
    let half_height = self.height / 2.0;
    let half_width = self.aspect * half_height;
    OrthoCamera {

      eye: self.eye,
      target: self.target,
      up: self.up,
      view_matrix: Matrix4::look_at(self.eye, self.target, self.up),

      height:   self.height,
      aspect:  self.aspect,
      near:   self.near,
      far:    self.far,
      projection_matrix: Matrix4::from(Ortho {
        top:    half_height,
        bottom: -half_height,
        right:  half_width,
        left:   -half_width,
        near:   self.near,
        far:    self.far,
      }),
    }
  }
}