# Raylib-rs-ext
Please note that I'm not affiliated with raylib-rs, I thought it'd be a good idea to port some more stuff from the original raylib, now here we are.

# Examples
```rust
use raylib::prelude::*;
use raylib_ext::*;

fn main() {
  let (mut rl, thread) = init()
    .size(640, 480)
    .title("Hello, World")
    .build();

  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);

    // Draws text where (x, y) is in the middle of the text instead of top-left
    d.draw_centered_text("Hello, world!", 12, 12, 20, Color::BLACK);
  }
}
```

```rust
use raylib::prelude::*;
use raylib_ext::*;

fn main() {
  let (mut rl, thread) = init()
    .size(640, 480)
    .title("Hello, World")
    .build();

  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);

    // Draws text where (x, y) is in the top-right corner instead of top-left
    d.draw_right_aligned_text("Hello, world!", 12, 12, 20, Color::BLACK);
  }
}
```
