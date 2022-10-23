extern crate cursive;

use std::io;
use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};

pub fn start() -> Result<(), io::Error> {
    let mut siv = Cursive::new();


    siv.add_layer(TextView::new("Hello World!\nPress q to quit."));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}