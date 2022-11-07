pub mod cursive_tui;

use cursive_tui::*;

use std::io;

fn main() -> Result<(), io::Error> {
    text_area::start()
}
