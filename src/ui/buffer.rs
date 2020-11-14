use crate::lang;
use crate::lang::{Number, Token};
use gtk::TextBufferExt;
use std::sync::Arc;

pub struct Buffer {
    pub source_buffer: Arc<sourceview::Buffer>,
    pub eval_buffer: Arc<sourceview::Buffer>,
}

impl Buffer {
    pub fn new() -> Buffer {
        let source_buffer = new_sourceview_buffer();
        let eval_buffer = new_sourceview_buffer();
        connect_eval_on_change(source_buffer.clone(), eval_buffer.clone());

        Buffer {
            source_buffer: source_buffer,
            eval_buffer: eval_buffer,
        }
    }
}

fn new_sourceview_buffer() -> Arc<sourceview::Buffer> {
    Arc::new(sourceview::Buffer::new::<gtk::TextTagTable>(None))
}

fn connect_eval_on_change(source: Arc<sourceview::Buffer>, eval: Arc<sourceview::Buffer>) {
    source.connect_changed(move |text_buffer| {
        let mut eval_text = "".to_owned();
        for i in 0..text_buffer.get_line_count() {
            eval_line(&mut eval_text, text_buffer, i);
        }

        eval.set_text(eval_text.as_ref())
    });
}

fn eval_line(eval_text: &mut String, text_buffer: &sourceview::Buffer, line_number: i32) {
    let input = get_line_input(text_buffer, line_number);
    if let Ok(tokens) = lang::evaluate(input.as_str()) {
        let line_result = find_expr_number(tokens)
            .map(|number| number.as_string())
            .unwrap_or("".to_string());
        eval_text.push_str(format!("{}\n", line_result).as_ref());
    } else {
        eval_text.push_str("\n");
    }
}

fn get_line_input(text_buffer: &sourceview::Buffer, line_number: i32) -> String {
    let start_line = text_buffer.get_iter_at_line(line_number);
    let mut end_line = start_line.clone();
    end_line.forward_to_line_end();

    text_buffer
        .get_text(&start_line, &end_line, false)
        .map(|gstring| gstring.to_string())
        .unwrap_or("".to_string())
}

fn find_expr_number(tokens: Vec<Token>) -> Option<Number> {
    tokens.iter().find_map(|token| match token {
        Token::Expr(number) => Some(*number),
        Token::Text(_) => None,
    })
}
