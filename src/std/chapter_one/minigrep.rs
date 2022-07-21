use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub word: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let word = args[3].clone();

        Ok(Config {
            query,
            filename,
            word,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(&config.word) {
            println!("word {}, found at line {}", config.word, index,);
        }
    }
    Ok(())
}
