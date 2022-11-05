use std::io;
use cursive::{Cursive, CursiveExt};
use cursive::views::TextView;

pub fn start() -> Result<(), io::Error> {
    // create a Cursive program
    let mut siv = Cursive::new();

    // add a TextView layer(window) to the program
    siv.add_layer(
        // this view can shows some texts to the terminal.
        // (texts to show are as the field 'content' to instantiate the TextView)
        TextView::new("Hello, world!\nPress 'q' to exit.")
    );

    // add a callback function to the program
    // it means that the program will quit when user presses the key 'q'
    siv.add_global_callback('q', |s| s.quit());

    // ^^^^^^^ Settings to the program ^^^^^^^

    // run the program
    siv.run();

    Ok(())
}