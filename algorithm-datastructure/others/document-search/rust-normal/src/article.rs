use std::io;
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

    pub fn parse(&self) -> io::Result<String> {
        let mut contents = String::new();
        let mut file = File::open(self.path.clone())?;
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn contains(&self, keyword: &str) -> io::Result<bool> {
        let contents = self.parse()?;

        // 方法1のやりかた
        let found = contents.contains(keyword);

        // 方法2のやりかた
        let re = Regex::new(r"\W").expect("Invalid Regex");
        let found = re.split(&contents)
            .map(|x| x.to_string())
            .any(|word| word.to_lowercase() == keyword.to_lowercase());


        Ok(found)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_when_file_not_exist() {
        let article = Article::new(PathBuf::from(""));
        assert!(article.parse().is_err());
    }

    // contains return true for capital tHe
    #[test]
    fn contains_work_case_insensitively() {
        let sample_file: PathBuf = PathBuf::from("tests/HelloWorld.txt");
        let article = Article::new(sample_file);
        let keyword = "tHe";
        assert!(article.contains(keyword).unwrap());
    }

    #[test]
    fn contains_return_false_when_not_found() {
        let sample_file: PathBuf = PathBuf::from("tests/HelloWorld.txt");
        let article = Article::new(sample_file);
        let keyword = "XXX";
        assert!(!article.contains(keyword).unwrap());
    }

    #[test]
    fn contains_return_false_on_partial_match() {
        let sample_file: PathBuf = PathBuf::from("tests/HelloWorld.txt");
        let article = Article::new(sample_file);
        let keyword = "t";
        assert!(!article.contains(keyword).unwrap());
    }
}
