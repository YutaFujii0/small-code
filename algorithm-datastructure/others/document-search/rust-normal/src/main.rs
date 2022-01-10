use std::env;

mod errors;
mod article;
mod article_search_service {
    pub mod service;
    pub mod error;
}

use crate::errors::*;
use crate::article_search_service::service::ArticleSearchService;

const ARTICLE_PATH: &str = "../articles-202109/";

fn main() {
    match execute() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}

fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let keyword = env::args()
        .nth(1)
        .ok_or(NoArgumentError)?;

    let mut service = ArticleSearchService::new(&keyword, ARTICLE_PATH)?;
    service.call()?;
    Ok(())
}
