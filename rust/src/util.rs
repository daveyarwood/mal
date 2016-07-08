use regex::Regex;

pub fn escape(string: &str) -> String {
    let mut escaped = String::new();
    for c in string.chars() {
        let _ = match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\x08' => escaped.push_str("\\b"),
            '\x0c' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(c),
        };
    };

    escaped
}

pub fn unescape(string: &str) -> String {
    // \" => "
    let step1 = Regex::new(r#"\\""#).unwrap().replace_all(string, "\"");
    // \n => (newline)
    let step2 = Regex::new(r"\\n").unwrap().replace_all(&step1, "\n");
    // \\ => \
    let step3 = Regex::new(r"\\\\").unwrap().replace_all(&step2, "\\");

    step3
}
