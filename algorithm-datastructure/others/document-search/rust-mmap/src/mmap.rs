use memmap::Mmap;
use std::fs;
use std::ops::DerefMut;
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let file = fs::File::open(path)?;
    let _mmap = unsafe { Mmap::map(&file)? };
    // println!("{:?}", String::from_utf8(mmap[..].to_vec()));

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

pub fn intensive_write(path: impl AsRef<Path>) -> std::io::Result<()> {
    use std::io::Write;

    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let mut mut_mmap = mmap.make_mut()?;

    // mut_mmap.deref_mut().write_all(b"Hello world!")?;
    Ok(())
}