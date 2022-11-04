extern crate cursive;

use std::io;
use cursive::{Cursive, CursiveExt};
use cursive::align::HAlign;
use cursive::event::Event::Char;
use cursive::views::{Dialog, SelectView, TextView};

pub fn start() -> Result<(), io::Error> {
    let mut time_select = SelectView::new().h_align(HAlign::Center);
    time_select.add_item("Short", 5);
    time_select.add_item("Medium", 12);
    time_select.add_item("Long", 25);

    time_select.set_on_submit(|s, time| {
        let x = s.pop_layer().unwrap();
        let text = format!("You will wait for {} seconds...", time);
        s.add_layer(
            Dialog::around(TextView::new(text)).dismiss_button("close")
        );

        s.add_layer(x)
    });

    let mut siv = Cursive::new();
    siv.add_layer(Dialog::around(time_select).title("How long is your wait?"));

    siv.run();
    Ok(())
}