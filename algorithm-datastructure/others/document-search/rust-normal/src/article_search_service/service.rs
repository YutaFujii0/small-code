use std::fs;
use regex::Regex;

use crate::article::Article;
use super::error::{Result, ArticleSearchServiceError};

pub struct ArticleSearchService<'a> {
    keyword: &'a str,
    target_dir: &'a str,
    hits: Vec<crate::article::Article>,
}

impl<'a> ArticleSearchService<'a> {
    pub fn new(keyword: &'a str, target_dir: &'a str) -> Result<Self> {
        Self::validate(keyword)?;
        Ok(Self {
            keyword,
            target_dir,
            hits: Vec::<Article>::new(),
        })
    }

    fn validate(keyword: &str) -> Result<()> {
        let validator = Regex::new(r"\w+").expect("Invalid Regex");
        if !validator.is_match(keyword) {
            return Err(ArticleSearchServiceError::InvalidArgument.into())
        }
        Ok(())
    }

    fn print_start_msg(&self) -> () {
        println!("Searching for {}", self.keyword);
    }

    fn print_result(&self) -> () {
        for (index, article) in self.hits.iter().enumerate() {
            println!("{}. {}", index + 1, article.title);
        }
        println!("Total: {}", self.hits.len());
    }

    fn search(&mut self) -> Result<()> {
        for entry in fs::read_dir(self.target_dir)? {
            let entry = entry?;
            let article = Article::new(entry.path());
            if article.contains(self.keyword)? {
                self.hits.push(article)
            }
        }
        Ok(())
    }

    pub fn call(&mut self) -> Result<()> {
        self.print_start_msg();
        self.search()?;
        self.print_result();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // invalid for ?, . and whitespace
    // test hits
    #[test]
    fn service_return_err_with_invalid_keyword() {
        let keyword1 = "?";
        let result1 = ArticleSearchService::new(keyword1, "");
        let keyword2 = ".";
        let result2 = ArticleSearchService::new(keyword2, "");
        let keyword3 = " ";
        let result3 = ArticleSearchService::new(keyword3, "");
        assert!(result1.is_err());
        assert!(result2.is_err());
        assert!(result3.is_err());
    }

    #[test]
    fn service_initialized_with_valid_keyword() {
        let keyword1 = "the";
        let result1 = ArticleSearchService::new(keyword1, "");
        let keyword2 = "日本語";
        let result2 = ArticleSearchService::new(keyword2, "");
        let keyword3 = "word2vec";
        let result3 = ArticleSearchService::new(keyword3, "");
        let keyword4 = "What?";
        let result4 = ArticleSearchService::new(keyword4, "");
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        assert!(result4.is_ok());
    }

    #[test]
    fn search_works() {
        let keyword = "the";
        let target_dir = "tests/";
        let mut service = ArticleSearchService::new(keyword, target_dir).unwrap();
        service.call().unwrap();
        assert_eq!(service.hits.len(), 1);
    }
}