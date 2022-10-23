mod tui_rs;
use tui_rs::helloworld;

use std::io;

fn main() -> Result<(), io::Error> {
    helloworld::start()
}