use raylib::prelude::*;

/// Trait for drawing text with different alignments (centered or right-aligned)
pub trait TextAlignment {
  #[allow(dead_code)] fn draw_centered_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) -> ();
  #[allow(dead_code)] fn draw_right_aligned_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) -> ();
  #[allow(dead_code)] fn measure_text_ext(&mut self, text: &str, font_size: i32) -> (i32, i32);
}

/// Implement the `TextAlignment` trait for `RaylibDrawHandle` to enable custom text drawing
impl TextAlignment for RaylibDrawHandle<'_> {
  /// Draw text centered at the specified (x, y) position
  ///
  /// # Example:
  /// ```rust
  /// use raylib::prelude::*;
  /// use raylib_ext::*;
  ///
  /// fn main() {
  ///   let (mut rl, thread) = init()
  ///     .size(640, 480)
  ///     .title("Hello, World")
  ///     .build();
  ///
  ///   while !rl.window_should_close() {
  ///     let mut d = rl.begin_drawing(&thread);
  ///
  ///     d.clear_background(Color::WHITE);
  ///
  ///     // Draws text where (x, y) is in the middle of the text instead of top-left
  ///     d.draw_centered_text("Hello, world!", 12, 12, 20, Color::BLACK);
  ///   }
  /// }
  /// ```
  #[allow(dead_code)]
  fn draw_centered_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) -> () {
    let text_width = self.measure_text(text, font_size);
    let final_x = x - (text_width / 2);  // Center horizontally
    let final_y = y - (font_size / 2);   // Center vertically (based on font size)
    self.draw_text(text, final_x, final_y, font_size, color);
  }

  /// Draw text right-aligned at the specified (x, y) position
  ///
  /// # Example:
  /// ```rust
  /// use raylib::prelude::*;
  /// use raylib_ext::*;
  ///
  /// fn main() {
  ///   let (mut rl, thread) = init()
  ///     .size(640, 480)
  ///     .title("Hello, World")
  ///     .build();
  ///
  ///   while !rl.window_should_close() {
  ///     let mut d = rl.begin_drawing(&thread);
  ///
  ///     d.clear_background(Color::WHITE);
  ///
  ///     // Draws text where (x, y) is in the top-right corner instead of top-left
  ///     d.draw_right_aligned_text("Hello, world!", 12, 12, 20, Color::BLACK);
  ///   }
  /// }
  /// ```
  #[allow(dead_code)]
  fn draw_right_aligned_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) -> () {
    let text_width = self.measure_text(text, font_size);
    let final_x = x - text_width;  // Align text to the right
    self.draw_text(text, final_x, y, font_size, color);
  }

  fn measure_text_ext(&mut self, text: &str, font_size: i32) -> (i32, i32) {
    (self.measure_text(text, font_size), font_size)
  }
}