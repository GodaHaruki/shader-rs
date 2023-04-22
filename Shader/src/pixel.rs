#[derive(Clone)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8,
    depth: u8,
}

impl Pixel {
  fn new() -> Self {
    Pixel{
      0,
      0,
      0,
      0,
    }
  }
}
