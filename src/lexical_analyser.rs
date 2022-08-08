use std::collections::HashSet;
pub use std::io::Read;

pub fn detect_keywords(content: &String) -> HashSet<String> {
    let keywords: Vec<&str> = vec![
        "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
        "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return",
        "short", "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned",
        "void", "volatile", "while", "printf", "scanf", "%d", "include", "stdio.h", "main",
    ];
    let mut result = HashSet::new();
    for w in content.split_whitespace() {
        if keywords.contains(&w) {
            result.insert(w.to_string());
        }
    }
    result
}

pub fn detect_numbers(content: &String) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut number = String::new();
    for c in content.chars() {
        if c.is_digit(10) {
            number.push(c);
        } else {
            if !number.is_empty() {
                result.insert(number);
                number = String::new();
            }
        }
    }
    result
}

pub fn detect_operators(content: &String) -> HashSet<String> {
    let operators: Vec<&str> = vec![
        "=", "!", "~", "++", "--", "+", "-", "*", "/", "%", "<<", ">>", "^", ",", ";", "<", ">",
        "<=", ">=", "==", "!=", "&&", "||", "?", "...",
    ];
    let mut result = HashSet::new();
    for operator in operators {
        if content.contains(operator) {
            result.insert(operator.to_string());
        }
    }
    result
}

pub fn detect_delimiters(content: &String) -> HashSet<String> {
    let delimiters: Vec<&str> = vec![
        "{", "}", "(", ")", "[", "]", ".", "&", "|", ",", "#", ";", ":", "",
    ];
    let mut result = HashSet::new();
    for delimiter in delimiters {
        if content.contains(delimiter) {
            result.insert(delimiter.to_string());
        }
    }
    result
}

pub fn detect_identifiers(content: &mut String) -> HashSet<String> {
    let keywords = detect_keywords(content);
    let operators = detect_operators(content);
    let delimiters = detect_delimiters(content);
    let numbers = detect_numbers(content);

    let mut result = HashSet::new();
    let mut identifiers = HashSet::new();
    identifiers.extend(keywords);
    identifiers.extend(operators);
    identifiers.extend(delimiters);
    identifiers.extend(numbers);

    for w in content.split_whitespace() {
        if !identifiers.contains(&w.to_string()) {
            // Check if it is a number with a semicolon at the end
            if !w.ends_with(";") && !w.starts_with(detect_numbers(content).iter().next().unwrap()) {
                result.insert(w.to_string());
            }
        }
    }
    result
}
