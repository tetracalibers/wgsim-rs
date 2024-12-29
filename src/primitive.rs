use winit::dpi::PhysicalSize;

#[derive(Debug, Clone, Copy)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

impl Size {
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }
}

impl Into<Size> for PhysicalSize<u32> {
  fn into(self) -> Size {
    Size {
      width: self.width,
      height: self.height,
    }
  }
}
