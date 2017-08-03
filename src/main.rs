extern crate piston_window;
extern crate sdl2_window;
extern crate conrod;

use piston_window::*;
use sdl2_window::Sdl2Window;
use conrod::{ Ui, UiBuilder, Theme };

fn main() {

println!("
~~~~ HYDRO v0.1.0 ~~~~\n\
======================\n\
This is basic as fuck.\n\
");

// Main project initializations.
// 1. Setup the engine.
// 2. Setup the window.
// 3. Profit $$$$.

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Hydro v0.1.0", [1024, 768])
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();

    window.set_capture_cursor(false);

    let mut ui = UiBuilder::new([1.0; 2]).build();

    while let Some(event) = window.next() {

        event.update(|_| {
            ui.set_widgets();
        });

        window.draw_2d(&event, |context, graphics| {
            ui.draw();
        });

        // if let Some(args) = e.render_args() {
        //
        //     window.draw_2d(|c,g| {});
        //     ui.draw();
        //     // println!("{:?}", args.draw_size());
        //     // println!("Render: {:?}", &args);
        //     // game.render(args);
        // }
        //
        // if let Some(args) = e.update_args() {
        //     ui.set_widgets(|ui| {
        //         Button::new()
        //             .color(color::BLUE)
        //             .top_left()
        //             .w_h(60.0, 30.0)
        //             .label("Test")
        //             .react(|| {})
        //             .ui(&mut ui);
        //     })
        //     ui.draw();
        // }
        //
        // if let Some(args) = e.press_args() {
        //
        //     // game.handle_input(args);
        //     println!("Input: {:?}", &args);
        // }
    }
}
