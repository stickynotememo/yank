use std::path::{ PathBuf, Path };
use std::path;
use std::fs::{ metadata, rename, File, Metadata };
use std::env;
use clap::builder::OsStr;
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

#[derive(PartialEq, Clone, Copy)]
enum MoveOp {
    Copy,
    Move
}

fn copy(args: &Args) {
    let file = PathBuf::from(args.file.as_ref().unwrap());
    // TODO: Add support for non UTF-8 characters by avoiding unwrap and using let else to
    // validate data.
    
    let Ok(value) = metadata(&file) else {
        panic!("Could not locate file or directory.");
    };
    if value.is_file() {
        // SAFETY:
        // Only one yank operation will run at once (it's assumed)
        // Therefore, calling set_var is safe.
        unsafe { env::set_var("YANK_FILE_CLIPBOARD", file.into_os_string().into_string().unwrap()) };
        // TODO: Add support for non UTF-8 characters by avoiding unwrap and using let else to
        // validate data.
        
    } else {
        todo!();
        // Use when directory copy is implemented
        // panic!("yank: Cannot copy \'{}\': Is a directory\nyank: Use -r to copy directories recursively", filename);
    }

}

fn paste(args: &Args) {
    // No file specified, yank should paste the file in the clipboard
    // Optionally, the --paste flag can be used to specify where to save the file

    // If either of these variables aren't set, yank hasn't been run yet in copy or cut mode.
    // TODO: Use a more readable method to convert to a static string (e.g. to_str())
    let moveop: MoveOp = match &env::var("YANK_FILE_MOVEOP").expect("yank: no file in clipboard")[..] {
        "MOVE" => MoveOp::Move,
        "COPY" => MoveOp::Copy,
        _ => {
            // If YANK_FILE_MOVEOP is set, no other value than the above two should be possible.
            unreachable!()
        }
    };
    let clipboard_string = &env::var("YANK_FILE_CLIPBOARD").expect("yank: no file in clipboard"); 
    let clipboard = Path::new(clipboard_string);

    let paste_file_name: &Path = match &args.paste_file {
        // If a paste file has been specified using the flag, it should be used instead
        // of the file used in the clipboard
        // If the clipboard value is being used, the file/directory should be pasted in the
        // current directory while maintaining its filename
        Some(paste_file) => Path::new(paste_file),
        None => { Path::new(clipboard.file_name().unwrap()) } 
        // Clipboard is set by yank, not by the user. Using unwrap is okay.
    };

    let paste_file_path = path::absolute(paste_file_name).expect("yank: paste path could not be parsed");

    match moveop {
        MoveOp::Move => { rename(clipboard, paste_file_path).expect("yank: filesystem error") },
        MoveOp::Copy => { todo!() }
    }

}

fn main() {
    let args = Args::parse();
    match &args.file {
        Some(file) => {
            copy(&args);
        },
        None => paste(&args)
    };

    
}
