// std
use std::ffi::{CStr};
// external crates
use gl::types::*;
use glutin::{GlContext, GlWindow, EventsLoop};
use cgmath::{ Rad, Deg, Matrix, Matrix4, Point3, Vector3 };
// modules
mod context;
use context::setup_context;
mod shader_program;
use shader_program::ShaderProgramBuilder;
mod event_handler;
mod game_state;
use game_state::GameState;
mod camera_builder;
use camera_builder::{ PerspectiveCameraBuilder };
mod triangle_creator;
use triangle_creator::add_triangle;
mod vao_builder;
use vao_builder::{VaoBuilder, AttribParameters};

fn main() {
  start_game();
}

fn start_game() {
  println!("Setting up window and events loop...");
  let (window, events_loop) = setup_context("Hello, Triangle", 1600, 900);
  print_gl_version();
  println!("Creating shader program...");
  let program_handle = ShaderProgramBuilder::new()
    .with_vertex_shader(include_str!("glsl/vertex.glsl"))
    .with_fragment_shader(include_str!("glsl/fragment.glsl"))
    .with_geometry_shader(include_str!("glsl/geometry.glsl"))
    .build();
  let mut game_state = GameState::new(program_handle);
  println!("Initializing game...");
  init_game(&mut game_state);
  println!("Running game...");
  run_game(window, events_loop, game_state);
}

fn print_gl_version() {
  let version = unsafe {
    let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
    String::from_utf8(data).unwrap()  // no semicolon means return
  };
  println!("OpenGL Version {}", version);
}

fn init_game(game_state: &mut GameState) {
  add_triangle(game_state);
  init_camera(game_state);
}

fn init_camera(game_state: &mut GameState) {
  game_state.camera = Some(PerspectiveCameraBuilder::new()
    .with_eye(Point3::new(0.0, 0.0, -2.0))
    .with_target(Point3::new(0.0, 0.0, 0.0))
    .with_up(Vector3::new(0.0, 1.0, 0.0))
    .with_fovy(Rad::from( Deg(45.0) ))
    .with_aspect(16.0/9.0)
    .with_near(0.1)
    .with_far(100.0)
    .build());
}

fn run_game(window: GlWindow, events_loop: EventsLoop, mut game_state: GameState) {
  let mut next_loop = events_loop;
  // ggez might have a useful timer, as well as other functionalities like sound
  // https://docs.rs/ggez/0.4.0/ggez/index.html
  loop {
    next_loop = event_handler::handle_events_loop(next_loop, &mut game_state);
    update(&mut game_state);
    draw(&game_state);
    window.swap_buffers().unwrap();
    if !game_state.running {
      break;
    }
  }
}

fn update(game: &mut GameState) {
  for entity_index in &game.entities {
    let model_matrix = game.model_matrices[*entity_index];
    let rot = Matrix4::from_angle_y(Rad(0.1));
    game.model_matrices[*entity_index] = rot * model_matrix;
  }
}

fn draw(game: &GameState) {
  unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); }
  let program = game.program_handle;
  if let Some(ref cam) = game.camera {
    unsafe {
      set_uniform_matrix(program, b"View\0", cam.view_matrix);
      set_uniform_matrix(program, b"Projection\0", cam.projection_matrix);
    }
  }
  for entity_index in &game.entities {
    let vao = game.vaos[*entity_index];
    let model_matrix = game.model_matrices[*entity_index];
    unsafe {
      set_uniform_matrix(program, b"Model\0", model_matrix);
      gl::BindVertexArray(vao);
      gl::DrawArrays(gl::POINTS, 0, 3);
    }
  }
}

// todo: move to shader program
unsafe fn set_uniform_matrix(program: GLuint, name: &[u8], matrix: Matrix4<GLfloat>) {
  // todo: store uniform locations at initialization
  let location = gl::GetUniformLocation(program, name.as_ptr() as *const _);
  gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
}
