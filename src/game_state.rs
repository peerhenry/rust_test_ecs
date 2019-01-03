use gl::types::*;
use cgmath::{ Matrix4 };
use camera_builder::PerspectiveCamera;

pub struct GameState {
  // assets
  pub running: bool,
  pub program_handle: GLuint,
  pub camera: Option<PerspectiveCamera>,
  // components
  pub vaos: Vec<GLuint>,
  pub model_matrices: Vec<Matrix4<GLfloat>>,
  // entity indices
  pub entities: Vec<usize>
}

impl GameState {
  pub fn new(program_handle: GLuint) -> GameState {
    GameState {
      running: true,
      program_handle,
      camera: None,
      vaos: Vec::new(),
      model_matrices: Vec::new(),
      entities: Vec::new()
    }
  }
}