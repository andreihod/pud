#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod lang;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let editor: sourceview::View = builder.get_object("sourceview").unwrap();

    editor.get_buffer().unwrap().connect_changed(|text_buffer| {
        let text = text_buffer
            .get_text(
                &text_buffer.get_start_iter(),
                &text_buffer.get_end_iter(),
                false,
            )
            .unwrap()
            .to_string();
        println!("{:#?}", lang::evaluate(text.as_str()));
    });

    window.show_all();

    gtk::main();
}
