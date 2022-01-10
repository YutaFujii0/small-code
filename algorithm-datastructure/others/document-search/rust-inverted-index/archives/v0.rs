use std::fs;
use heed::{EnvOpenOptions, Database};
use heed::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ArticleTitles {
    articles: Vec<String>,
}

fn main() -> Result<(), heed::Error> {
    println!("Hello, world!");

    // read dir

    // iterate each file
    //   - read file content
    //   - insert invert index
    fs::create_dir_all("target/heed.mdb")?;
    let env = EnvOpenOptions::new().open("target/heed.mdb")?;

    // We open the default unamed database.
    // Specifying the type of the newly created database.
    // Here we specify that the key is an str and the value a simple integer.
    let db: Database<Str, SerdeJson<ArticleTitles>> = env.create_database(None)?;

    // We then open a write transaction and start writing into the database.
    // All of those puts are type checked at compile time,
    // therefore you cannot write an integer instead of a string.
    let mut wtxn = env.write_txn()?;
    db.put(&mut wtxn, "seven", &ArticleTitles{ articles: vec!["hi".to_string()] })?;
    db.put(&mut wtxn, "zero", &ArticleTitles{ articles: vec!["hi".to_string()] })?;
    db.put(&mut wtxn, "five", &ArticleTitles{ articles: vec!["hi".to_string()] })?;
    db.put(&mut wtxn, "three", &ArticleTitles{ articles: vec!["hi".to_string()] })?;
    wtxn.commit()?;

    // We open a read transaction to check if those values are available.
    // When we read we also type check at compile time.
    let rtxn = env.read_txn()?;

    let ret = db.get(&rtxn, "zero")?;
    println!("{:?}", ret);

    let ret = db.get(&rtxn, "five")?;
    println!("{:?}", ret);

    Ok(())
}
