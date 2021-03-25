#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod lang;
mod ui;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let source_editor: sourceview::View = builder.get_object("source").unwrap();
    let eval_editor: sourceview::View = builder.get_object("eval").unwrap();

    let buffer = ui::Buffer::new();

    let source_style_context = source_editor.get_style_context();
    let eval_style_context = eval_editor.get_style_context();
    let css_provider = gtk::CssProvider::new();

    css_provider
        .load_from_data("textview { font-family: Monospace; font-size: 16pt; }".as_ref())
        .unwrap();

    source_style_context.add_provider(&css_provider, 1);
    eval_style_context.add_provider(&css_provider, 1);

    source_editor.set_buffer(Some(buffer.source_buffer.as_ref()));
    eval_editor.set_buffer(Some(buffer.eval_buffer.as_ref()));

    window.show_all();

    gtk::main();
}
