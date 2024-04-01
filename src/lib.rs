use std::error::Error;
use std::fs::read_to_string;

pub struct Config {
    pub query: String,
    pub file_p: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed ");
        }
        let query = args[1].clone();
        let file_p = args[2].clone();
        Ok(Config { query, file_p })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(&config.file_p)?;
    // println!(
    //     "Searched for {}\nin file => {}\n\n{}\n",
    //     config.query, &config.file_p, contents
    // );
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = std::vec::Vec::new();
    for line in contents.lines(){
       if line.contains(query){
            res.push(line);
       }
    }
    res
}