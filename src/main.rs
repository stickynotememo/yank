// TODO: Clipboard history
// TODO: Prevent deletion of yanked files
// TODO: Directory support

use std::path::{ PathBuf };
use std::path;
use std::fs;
use clap::Parser;
use preferences::{ AppInfo, Preferences };
use serde::{ Serialize, Deserialize };

#[cfg(test)]
mod tests;

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

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
enum MoveOp {
    Copy,
    Move
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct UserData {
    moveop: MoveOp,
    object_path: PathBuf
}

const PREFS_KEY: &str = "/etc/yank/clipboard";
const APP_INFO: AppInfo = AppInfo {
    name: "yank",
    author: "stickynotememo"
};

fn copy(args: &Args) {
    let file = PathBuf::from(&args.file.as_ref().unwrap());
    let file_metadata = fs::metadata(&file).expect("Could not find file or folder");

    let moveop = match args.cut {
        true => MoveOp::Move,
        false => MoveOp::Copy
    };

    if file_metadata.is_file() {
        // Copy only specifies the file and saves it in storage. The file operation is
        // done by paste()
        let user_data = UserData {
            moveop: moveop,
            object_path: path::absolute(file).expect("Could not find file or folder")
        };
        user_data.save(&APP_INFO, PREFS_KEY).expect("Could not save clipboard to file.");

    } else {
        todo!();
        // Use when directory copy is implemented
        // panic!("yank: Cannot copy \'{}\': Is a directory\nyank: Use -r to copy directories recursively", filename);
    }

}

fn paste(args: &Args) {
    // No file specified, yank should paste the file in the clipboard
    // Optionally, the --paste flag can be used to specify where to save the file

    let user_data = UserData::load(&APP_INFO, PREFS_KEY).unwrap();
    let moveop = user_data.moveop;
    let clipboard = user_data.object_path;

    let paste_file_name: PathBuf = match &args.paste_file {
        // If a paste file has been specified using the flag, it should be used instead
        // of the file used in the clipboard
        // If the clipboard value is being used, the file/directory should be pasted in the
        // current directory while maintaining its filename
        Some(paste_file) => PathBuf::from(paste_file),
        None => PathBuf::from(path::absolute(clipboard.file_name().unwrap()).unwrap()) 
        // Clipboard is set by yank, not by the user. Using unwrap is okay.
    };

    let paste_file_path = path::absolute(paste_file_name).expect("yank: paste path could not be parsed");

    match moveop {
        MoveOp::Move => { fs::rename(clipboard, paste_file_path).expect("yank: an error occurred while copying files"); },
        MoveOp::Copy => { fs::copy(clipboard, paste_file_path).expect("yank: an error occurred while copying files"); }
    };

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
