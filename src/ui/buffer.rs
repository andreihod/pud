use crate::lang;
use crate::lang::{Number, Token};
use gtk::TextBufferExt;
use regex::Regex;
use sourceview::{BufferExt, StyleSchemeManagerExt};
use std::sync::Arc;

fn is_new_line(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\n]+").unwrap();
    }
    RE.is_match(text)
}

pub struct Buffer {
    pub source_buffer: Arc<sourceview::Buffer>,
    pub eval_buffer: Arc<sourceview::Buffer>,
}

impl Buffer {
    pub fn new() -> Buffer {
        let source_buffer = new_sourceview_buffer();
        let eval_buffer = new_sourceview_buffer();
        connect_eval_on_change(source_buffer.clone(), eval_buffer.clone());

        let manager = sourceview::StyleSchemeManager::new();

        let source_scheme = manager.get_scheme("solarized-dark").unwrap();
        let eval_scheme = manager.get_scheme("solarized-light").unwrap();

        source_buffer.set_style_scheme(Some(&source_scheme));
        eval_buffer.set_style_scheme(Some(&eval_scheme));

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
        let mut iter_start = text_buffer.get_start_iter();

        loop {
            let mut iter_end = iter_start.clone();
            iter_end.forward_to_line_end();

            if iter_end == iter_start {
                break;
            }

            let line = text_buffer
                .get_text(&iter_start, &iter_end, false)
                .map(|gstring| gstring.to_string())
                .unwrap_or("".to_string());

            eval_text.push_str(&eval_line(line));
            iter_start.forward_line();
        }

        eval.set_text(eval_text.as_ref())
    });
}

fn eval_line(input: String) -> String {
    if is_new_line(&input) {
        return "\n".to_string();
    }

    if let Ok(tokens) = lang::evaluate(input.as_str()) {
        let line_result = find_expr_number(tokens)
            .map(|number| number.as_string())
            .unwrap_or("".to_string());
        format!("{}\n", line_result)
    } else {
        "\n".to_string()
    }
}

fn find_expr_number(tokens: Vec<Token>) -> Option<Number> {
    tokens.iter().find_map(|token| match token {
        Token::Expr(number) => Some(*number),
        Token::Text(_) => None,
    })
}
