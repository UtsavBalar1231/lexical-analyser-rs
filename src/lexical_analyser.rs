use std::collections::{HashMap, HashSet};

const KEYWORDS: &[&str] = &[
    "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
    "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void",
    "volatile", "while", "printf", "scanf", "%d", "include", "stdio.h", "main",
];
const OPERATORS: &[&str] = &["=", "!", "~", "+", "-", "*", "/", "%", "^", ","];
const DELIMITERS: &[&str] = &[
    "{", "}", "(", ")", "[", "]", ".", "&", "|", ",", "#", ";", ":", "",
];

pub fn parse(lines: &mut Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut keywords = HashSet::new();
    let mut operators = HashSet::new();
    let mut delimiters = HashSet::new();
    let mut identifiers = HashSet::new();
    let mut constants = HashSet::new();

    for word in lines.clone() {
        if KEYWORDS.contains(&word.as_str()) {
            keywords.insert(word.to_string());
        }
    }

    for word in lines.clone() {
        if OPERATORS.contains(&word.as_str()) {
            operators.insert(word.to_string());
        }
    }

    for word in &lines.clone() {
        if DELIMITERS.contains(&word.as_str()) {
            delimiters.insert(word.to_string());
        }
    }

    for word in lines.clone() {
        let mut number = String::new();
        for c in word.chars() {
            if c.is_digit(10) {
                number.push(c);
            } else {
                if !number.is_empty() {
                    constants.insert(number);
                    number = String::new();
                }
            }
        }
    }

    for word in lines.clone() {
        if word.chars().all(|c| c.is_alphanumeric()) {
            identifiers.insert(word.to_string());
        }
    }

    let mut result = HashMap::new();
    result.insert("keywords".to_string(), keywords);
    result.insert("operators".to_string(), operators);
    result.insert("delimiters".to_string(), delimiters);
    result.insert("identifiers".to_string(), identifiers);
    result.insert("constants".to_string(), constants);
    result
}
