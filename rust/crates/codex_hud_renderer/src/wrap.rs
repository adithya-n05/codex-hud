use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn wrap_line_unicode_safe(line: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![line.to_string()];
    }

    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();

    for grapheme in UnicodeSegmentation::graphemes(line, true) {
        let candidate = format!("{current}{grapheme}");
        if UnicodeWidthStr::width(candidate.as_str()) > width && !current.is_empty() {
            out.push(current.clone());
            current.clear();
        }
        current.push_str(grapheme);
    }

    if !current.is_empty() {
        out.push(current);
    }

    if out.is_empty() {
        vec![String::new()]
    } else {
        out
    }
}

pub fn wrap_text(input: &str, width: usize) -> Vec<String> {
    wrap_line_unicode_safe(input, width)
}
