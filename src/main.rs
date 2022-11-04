pub mod tui_rs;
pub mod cursive_tui;

use tui_rs::helloworld;

use std::io;

fn main() -> Result<(), io::Error> {
    helloworld::start()
}