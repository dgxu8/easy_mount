use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::io;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "CLI for easy mount/umounting", long_about = None)]
struct Cli {
    /// Path to device you want to mount/umount
    target: std::path::PathBuf,
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    println!("got {:?}", args.target);

    let meta = fs::metadata(args.target)?;
    let file_type = meta.file_type();
    if file_type.is_block_device() {
        println!("It's a block device");
    } else {
        println!("Not a block device");
    }
    Ok(())
}
