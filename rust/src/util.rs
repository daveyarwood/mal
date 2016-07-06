use regex::Regex;

pub fn escape(string: &str) -> String {
    // " => \"
    let step1 = Regex::new(r#"""#).unwrap().replace_all(string, "\\\"");
    // (newline) => \n
    let step2 = Regex::new(r"\n").unwrap().replace_all(&step1, "\\n");
    // \ => \\
    // let step3 = Regex::new(r"\\").unwrap().replace_all(&step2, "\\\\");

    step2
    // step3
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
