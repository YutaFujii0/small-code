use std::env;
use crate::index::Index;

mod article;
mod errors;
mod import;
mod index;
mod search;

use article::*;
use errors::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyword = env::args()
        .nth(1)
        .ok_or(NoArgumentError)?;

    if keyword == "--import" {
        import::import_documents()?;
    } else {
        search::search(&keyword)?;
    }
    Ok(())
}

#[allow(dead_code)]
fn execute() -> Result<(), heed::Error> {
    let index = Index::<Articles>::new()?;

    let rtxn = index.env.read_txn()?;
    let ret = index.main.get(&rtxn, "world")?;
    match ret {
        Some(v) => {
            for (index, article) in v.articles.iter().enumerate() {
                println!("{}. {}", index + 1, article.title);
            }
        },
        None => {
            println!("Not found")
        },
    }

    Ok(())
}
