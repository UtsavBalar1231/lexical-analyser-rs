use lexical_analyser_rs::lexical_analyser;
use std::fs::File;
use std::io::Read;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let mut content = read_file("test.c").unwrap();

    let keywords = lexical_analyser::detect_keywords(&mut content);
    println!("keywords: {:?}", keywords);

    let operators = lexical_analyser::detect_operators(&mut content);
    println!("operators: {:?}", operators);

    let delimiters = lexical_analyser::detect_delimiters(&mut content);
    println!("delimiters: {:?}", delimiters);

    let numbers = lexical_analyser::detect_numbers(&mut content);
    println!("numbers: {:?}", numbers);

    let identifiers = lexical_analyser::detect_identifiers(&mut content);
    println!("identifiers: {:?}", identifiers);
}
