use lexical_analyser_rs::lexical_analyser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    Ok(lines)
}

fn main() {
    let lines = read_file("test.c").unwrap();

    let mut words = vec![];
    for word in lines.split_whitespace() {
        words.push(word.to_string());
    }

    let result = lexical_analyser::parse(&mut words);
    println!("{:#?}", result);

    // write to file
    write_to_file(&result);
}

fn write_to_file(result: &HashMap<String, HashSet<String>>) {
    for (key, value) in result {
        let filename: String = key.to_string() + ".txt";
        let mut file = File::create(filename).unwrap();
        for word in value {
            file.write_all(word.as_bytes()).unwrap();
            file.write_all(", ".as_bytes()).unwrap();
        }
        file.write_all("\n".as_bytes()).unwrap();
    }
}
