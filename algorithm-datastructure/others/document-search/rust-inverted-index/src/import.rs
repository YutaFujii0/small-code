use std::fs::{self, File};
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use regex::Regex;

use super::index::Index;
use super::article::{Articles, Article};

const ARTICLE_PATH: &str = "../articles-202109/";

pub fn import_documents() -> Result<(), heed::Error> {
  let index = Index::<Articles>::new()?;

  for entry in fs::read_dir(ARTICLE_PATH)? {
      let entry = entry?;

      // parse title
      let title = parse_title(&entry.path());
      // return a vector of unique words
      let words = unique(&entry.path())?;

      // insert index with word: title pair
      for word in words {
          let new_article = Article{ title: title.clone() };

          let rtxn = index.env.read_txn()?;
          let ret = index.main.get(&rtxn, &word)?;

          let articles = match ret {
              Some(mut v) => {
                  v.articles.push(new_article);
                  v
              },
              None => {
                  Articles{
                      articles: vec![new_article],
                  }
              },
          };

          let mut wtxn = index.env.write_txn()?;
          index.main.put(&mut wtxn, &word, &articles)?;
          wtxn.commit()?;
      }
  }

  let rtxn = index.env.read_txn()?;
  let ret = index.main.get(&rtxn, "foo")?;
  println!("{:?}", ret);

  Ok(())
}

fn parse_title(path: &PathBuf) -> String {
  let filename = path
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();
  let re = Regex::new(r"\.txt$").expect("Invalid Regex");
  re.replace(filename, "").to_string()
}

fn unique(path: &PathBuf) -> io::Result<Vec<String>> {
  let mut contents = String::new();
  let mut file = File::open(path)?;
  file.read_to_string(&mut contents)?;
  let re = Regex::new(r"\W").expect("Invalid Regex");
  // lowercase
  let mut words = re
      .split(&contents)
      .filter(|x| x.len() > 0)
      .map(|x| x.to_string().to_lowercase())
      .collect::<Vec<String>>();
  // merge sort
  words.sort();
  // dedup and remove duplicates
  words.dedup();
  Ok(words)
}