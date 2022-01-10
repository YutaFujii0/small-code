use std::env;
use regex::Regex;

mod errors;
mod article;
mod article_search_service;

use crate::errors::*;
use crate::article_search_service::ArticleSearchService;

const ARTICLE_PATH: &str = "../articles-202109/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyword = env::args().nth(1)
                                    .ok_or(NoArgumentError)?;

    let validator = Regex::new(r"\w+").expect("Invalid Regex");
    if !validator.is_match(&keyword) {
        return Err(InvalidArgumentError.into())
    }

    let mut service = ArticleSearchService::new(&keyword, ARTICLE_PATH);
    service.call()?;
    Ok(())
}

// ----------------------------------------------------

use std::fs;
use std::error;
use std::result::Result;

use crate::article::Article;

pub struct ArticleSearchService<'a> {
    keyword: &'a str,
    target_dir: &'a str,
    hits: Vec<crate::article::Article>,
}

impl<'a> ArticleSearchService<'a> {
    pub fn new(keyword: &'a str, target_dir: &'a str) -> Self {
        Self {
            keyword,
            target_dir,
            hits: Vec::<Article>::new(),
        }
    }

    pub fn call(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("Searching for {}", self.keyword);
        for entry in fs::read_dir(self.target_dir)? {
            let entry = entry?;
            let article = Article::new(entry.path());
            if article.contains(self.keyword).unwrap() {
                self.hits.push(article)
            }
        }
        for (index, article) in self.hits.iter().enumerate() {
            println!("{}. {}", index + 1, article.title);
        }
        println!("Total: {}", self.hits.len());
        Ok(())
    }
}


// ----------------------------------------------------

use std::error;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub struct Article {
    path: PathBuf,
    pub title: String,
}

impl Article {
    pub fn new(path: PathBuf) -> Self {
        let title = Self::parse_title(&path);
        Self { path, title }
    }

    pub fn parse(&self) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        let mut file = File::open(self.path.clone())?;
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn contains(&self, keyword: &str) -> Result<bool, Box<dyn error::Error>> {
        let text = self.parse()?;
        let re = Regex::new(r"\W").expect("Invalid Regex");
        let found = re.split(&text)
                          .map(|x| x.to_string())
                          .any(|word| word.to_lowercase() == keyword.to_lowercase());
        Ok(found)
    }

    fn parse_title(path: &PathBuf) -> String {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let re = Regex::new(r"\.txt$").expect("Invalid Regex");
        re.replace(filename, "").to_string()
    }
}



// ----------------------------------------------------


use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct NoArgumentError;

impl fmt::Display for NoArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No keyword given. Please provide one.\nExample:\n  search startup")
    }
}

impl error::Error for NoArgumentError {}

#[derive(Debug, Clone)]
pub struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keyword is invalid. Keyword can only contain alphabets and numbers.\nExample:\n  search word2vec")
    }
}

impl error::Error for InvalidArgumentError {}
