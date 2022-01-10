// use std::ascii::AsciiExt;
// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
// use regex::Regex;

// const ARTICLE_PATH: &str = "../articles-202109/";

// fn main() -> std::io::Result<()> {
//     let keyword = env::args().nth(1).unwrap();
//     println!("Searching for {} in HelloWorld.txt", keyword);
//     let filepath = format!("{}{}", ARTICLE_PATH, "HelloWorld.txt");
//     let mut file = File::open(filepath)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let re = Regex::new(r"\W").expect("Invalid Regex");
//     let words = re.split(&contents).map(|x| x.to_string()).collect::<Vec<String>>();
//     let found = words.into_iter().any(|x| x.to_lowercase() == keyword.to_lowercase());
//     println!("Result: {:?}", found);
//     Ok(())
// }