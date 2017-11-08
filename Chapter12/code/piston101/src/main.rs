extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            ellipse([0.0, 0.0, 1.0, 0.5],
                    [0.0, 0.0, 100.0, 100.0],
                    context.transform,
                    graphics);
        });
    }
}
