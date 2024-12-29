use std::future::Future;

use winit::event::WindowEvent;

use crate::ctx::DrawingContext;
use crate::primitive::Size;

#[allow(opaque_hidden_inferred_bound, unused_variables)]
pub trait Render<'a> {
  type Initial;

  fn new(
    ctx: &DrawingContext<'a>,
    initial: &Self::Initial,
  ) -> impl Future<Output = Self>;
  fn resize(&mut self, ctx: &mut DrawingContext, size: Size) {
    if size.width > 0 && size.height > 0 {
      ctx.resize(size);
    }
  }
  fn process_event(&mut self, event: &WindowEvent) -> bool {
    false
  }
  fn update(&mut self, ctx: &DrawingContext, dt: std::time::Duration) {}
  fn draw(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    sample_count: u32,
  ) -> Result<(), wgpu::SurfaceError>;
  fn submit(
    &self,
    queue: &wgpu::Queue,
    encoder: wgpu::CommandEncoder,
    frame: Option<wgpu::SurfaceTexture>,
  ) {
    queue.submit(std::iter::once(encoder.finish()));

    if let Some(frame) = frame {
      frame.present();
    }
  }
}
