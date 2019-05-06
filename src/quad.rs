use crate::rect;
use cgmath::*;
use gfx::pso::bundle::Bundle;
use gfx::{self, *};

pub type ColorFormat = format::Rgba8;
pub type DepthFormat = format::Depth;

gfx_defines! {
  vertex Vertex {
    pos: [f32; 2] = "a_Pos",
  }

  constant Locals {
    transform: [[f32; 4]; 4] = "u_Transform",
    color: [f32; 4] = "u_Color",
    z: f32 = "u_Z",
  }

  pipeline pipe {
    vbuf: VertexBuffer<Vertex> = (),
    locals: ConstantBuffer<Locals> = "Locals",
    out_color: BlendTarget<ColorFormat> = ("Target0", state::ColorMask::all(), preset::blend::ALPHA),
    out_depth: DepthTarget<DepthFormat> = preset::depth::LESS_EQUAL_WRITE,
  }
}

pub const QUAD: [Vertex; 4] = [
  Vertex { pos: [-1.0, 1.0] },
  Vertex { pos: [-1.0, -1.0] },
  Vertex { pos: [1.0, -1.0] },
  Vertex { pos: [1.0, 1.0] },
];
pub const QUAD_INDICES: [u16; 6] = [0u16, 1, 2, 2, 3, 0];

pub fn draw_quad<R: Resources, C: CommandBuffer<R>>(
  quad_bundle: &mut Bundle<R, pipe::Data<R>>,
  rect: &rect::Rect,
  color: [f32; 4],
  z: f32,
  window_dim: Vector2<f32>,
  encoder: &mut Encoder<R, C>,
) {
  let transform = rect.as_matrix_within_window(window_dim);

  let locals = Locals {
    color,
    transform: transform.into(),
    z,
  };
  encoder.update_constant_buffer(&quad_bundle.data.locals, &locals);
  quad_bundle.encode(encoder);
}
