use std::io::Read;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    // println!("{:?}", &contents);

    Ok(())
}

#[allow(dead_code)]
pub fn read_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        read_file(entry.path())?;
    }
    Ok(())
}