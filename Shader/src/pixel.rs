#[derive(Clone)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
  fn new(r: u8, g: u8, b: u8) -> Self {
    Pixel{
      r,
      g,
      b,
    }
  }
}
