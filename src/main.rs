extern crate tcod;

use tcod::console::{Console, Root, FontLayout, Renderer, BackgroundFlag, TextAlignment};
use tcod::colors;

fn main() {
    // println!("Hello, world!");
    let mut con = Root::initializer()
                    .size(80, 40)
                    .title("Micah was here")
                    .fullscreen(false)
                    .font("terminal.png", FontLayout::AsciiInCol)
                    .renderer(Renderer::SDL)
                    .init();

    con.set_default_background(colors::BLACK);
    con.set_default_foreground(colors::WHITE);
    con.print_ex(5, 5, BackgroundFlag::None, TextAlignment::Left, "Hello World");
    con.flush();

    while !con.window_closed() {
        // do stuff probably
    }
}
