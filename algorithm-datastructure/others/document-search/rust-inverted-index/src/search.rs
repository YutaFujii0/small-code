use super::index::Index;
use super::article::Articles;

pub fn search(keyword: &str) -> Result<(), heed::Error> {
    println!("Searching for {}", keyword);
    let index = Index::<Articles>::new()?;

    let rtxn = index.env.read_txn()?;
    let ret = index.main.get(&rtxn, keyword)?;
    match ret {
        Some(v) => {
            for (index, article) in v.articles.iter().enumerate() {
                println!("{}. {}", index + 1, article.title);
            }
            println!("Total: {}", v.articles.len());
        },
        None => {
            println!("Not found");
        },
    }

    Ok(())
}
