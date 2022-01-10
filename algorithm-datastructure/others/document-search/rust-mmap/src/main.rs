mod mmap;
mod io_read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = "../articles_all/";
    // mmap::read_dir(path)?;
    // io_read::read_dir(path)?;


    // let single_file_path = "../README.md";
    // mmap::read_file(single_file_path)?;
    // io_read::read_file(single_file_path)?;

    let single_file_path = "./mapfile";
    mmap::intensive_write(single_file_path)?;
    // io_read::read_file(single_file_path)?;



    Ok(())
}
