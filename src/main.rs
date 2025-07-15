use std::path::PathBuf; use std::fs::{ metadata, File, Metadata }; use std::env;
use clap::Parser;

// TODO: Clipboard history
// TODO: Prevent deletion of yanked files
// TODO: Directory support

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
    #[arg(short = 'x', long)]
    cut: bool,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long = "paste")]
    paste_file: Option<String>,
}

fn copy() {}
fn cut() {}
fn paste() {}

fn main() {
    let args = Args::parse();
    match &args.file {
        Some(file) => {
            // If paste file is also present, copy like normal
        }
        None => {
            // No file specified, yank should paste the file in the clipboard
            // Optionally, the --paste flag can be used to specify where to save the file
            let paste_file = match args.paste_file {
                Some(paste_file) => paste_file,
                None => { String::from("No paste_file provided") }
            };
            dbg!(paste_file);
        }
    };

    let file = PathBuf::from(args.file.unwrap());
    let filename = &file.file_name().unwrap().to_str().unwrap(); 
    // TODO: Add support for non UTF-8 characters by avoiding unwrap and using let else to
    // validate data.
    
    let Ok(value) = metadata(&file) else {
        panic!("Could not locate file or directory.");
    };

    if value.is_file() {
        unsafe { env::set_var("CLIPBOARD", file.into_os_string().into_string().unwrap()) };
        // TODO: Add support for non UTF-8 characters by avoiding unwrap and using let else to
        // validate data.
        
    } else {
        panic!("yank: Cannot copy \'{}\': Is a directory\nyank: Use -r to copy directories recursively", filename);
    }




    if args.recursive {

    }
    
}
