use std::error::Error;

use wgsim::app::App;
use wgsim::ctx::DrawingContext;
use wgsim::render::Render;

fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();

  let initial = setup();

  let mut app: App<State> = App::new("screen-fit-contain-image", initial);
  app.run()?;

  Ok(())
}

fn setup() -> Initial {
  Initial {}
}

struct Initial {}

struct State {}

impl<'a> Render<'a> for State {
  type Initial = Initial;

  async fn new(ctx: &DrawingContext<'a>, initial: &Self::Initial) -> Self {
    Self {}
  }

  fn draw(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    render_target: &wgpu::TextureView,
    sample_count: u32,
  ) -> Result<(), wgpu::SurfaceError> {
    Ok(())
  }
}
