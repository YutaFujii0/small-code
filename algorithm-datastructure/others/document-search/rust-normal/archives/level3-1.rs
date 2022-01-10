// use std::env;
// use std::error;
// use std::fs::{self, File};
// use std::io::prelude::*;
// use regex::Regex;
// use std::path::PathBuf;

// mod errors;

// const ARTICLE_PATH: &str = "../articles-202109/";

// fn main() -> std::result::Result<(), Box<dyn error::Error>> {
//     let keyword = env::args().nth(1)
//                                     .ok_or(errors::NoArgumentError)?;

//     let validator = Regex::new(r"\w+").expect("Invalid Regex");
//     if !validator.is_match(&keyword) {
//         return Err(errors::InvalidArgumentError.into())
//     }
//     println!("Searching for {}", keyword);
//     let mut hits = Vec::<PathBuf>::new();
//     for entry in fs::read_dir(ARTICLE_PATH)? {
//         let entry = entry?;
//         let contents = parse(entry.path())?;
//         if contains(&keyword, contents) { hits.push(entry.path()) }
//     }
//     for (index, path) in hits.iter().enumerate() {
//         println!("{}. {}", index + 1, filename_without_extension(path));
//     }
//     println!("Total: {}", hits.len());
//     Ok(())
// }

// fn parse(filepath: PathBuf) -> Result<String, std::io::Error> {
//     let mut file = File::open(filepath)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn contains(keyword: &str, text: String) -> bool {
//     let re = Regex::new(r"\W").expect("Invalid Regex");
//     let found = re.split(&text)
//                        .map(|x| x.to_string())
//                        .any(|word| word.to_lowercase() == keyword.to_lowercase()); found
// }

// fn filename_without_extension(path: &PathBuf) -> String {
//     let filename = path.file_name().unwrap().to_str().unwrap();
//     let re = Regex::new(r"\.txt$").expect("Invalid Regex");
//     re.replace(filename, "").to_string()
// }

// ----------------------
// // errors.rs
// use std::fmt;
// use std::error;

// #[derive(Debug, Clone)]
// pub struct NoArgumentError;

// impl fmt::Display for NoArgumentError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "No keyword given. Please provide one.\nExample:\n  search startup")
//     }
// }

// impl error::Error for NoArgumentError {}

// #[derive(Debug, Clone)]
// pub struct InvalidArgumentError;

// impl fmt::Display for InvalidArgumentError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Keyword is invalid. Keyword can only contain alphabets and numbers.\nExample:\n  search word2vec")
//     }
// }

// impl error::Error for InvalidArgumentError {}
