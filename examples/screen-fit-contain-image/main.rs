use std::error::Error;

use image::GenericImageView;

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
  let img_bytes = include_bytes!("../assets/img/pastel-tomixy.png");
  let image = image::load_from_memory(img_bytes).unwrap();
  let image_size = image.dimensions();

  Initial { image, image_size }
}

struct Initial {
  image: image::DynamicImage,
  image_size: (u32, u32),
}

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
