use rust_tui_learning::tui_rs::helloworld;

use std::io;

fn main() -> Result<(), io::Error> {
    helloworld::start()
}