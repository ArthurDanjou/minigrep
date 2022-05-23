use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub research: String,
    pub filename: String,
    pub verbose: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments !");
        }

        let research = args[1].clone();
        let filename = args[2].clone();

        let verbose = env::var("VERBOSE").is_err();

        Ok(Config {
            research,
            filename,
            verbose
        })
    }
}

pub fn search<'a>(
    research: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(&research) {
            result.push(line);
        }
    }

    result
}

pub fn search_insensitive_case<'a>(
    research: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let research = research.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&research) {
            result.push(line);
        }
    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("Content : \n{}", content);

    let result = if config.verbose {
        search(&config.research, &content)
    } else {
        search_insensitive_case(&config.research, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensible_case() {
        let research = "duct";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape.";

        assert_eq!(vec!["sécurité, rapidité, productivité."], search(research, content));
    }

    #[test]
    fn insensible_case() {
        let research = "rUsT";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            search_insensitive_case(research, content)
        );
    }
}
