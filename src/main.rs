pub mod tui_rs;
pub mod cursive_tui;

use cursive_tui::helloworld;

use std::io;

fn main() -> Result<(), io::Error> {
    helloworld::start()
}
