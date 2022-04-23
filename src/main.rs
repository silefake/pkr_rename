use std::env;
use std::any::type_name;
use std::path::PathBuf;
use std::fs;

use pkr_rename::rename_proc::{self, Op};

// https://stackoverflow.com/questions/57224310/when-i-run-my-rust-application-on-windows-the-coloring-works-with-cargo-run-but
// https://github.com/ogham/rust-ansi-term/blob/master/src/windows.rs
use ansi_term::enable_ansi_support;

fn main() -> std::io::Result<()> {
    enable_ansi_support().unwrap();
    sub_main()?;
    Ok(())
}

fn sub_main() -> std::io::Result<()> {
    let config = parse_args();
    println!("{:?}", config);

    let targets: Box<Vec<filename::Filename>> = list_files(&config.path);
    // println!("{:?}", targets);

    let results = match config.op {
        Op::Insert(text) => {
            rename_proc::insert(targets, &text)
        },
        Op::InsertTail(text) => {
            rename_proc::insert_tail(targets, &text)
        }, 
        Op::Remove(n) => {
            rename_proc::remove(targets, n)
        }, 
        Op::RemoveTail(n) => {
            rename_proc::remove_tail(targets, n)
        }
        Op::Replace(text_replaced, text_new) => {
            rename_proc::replace(targets, &text_replaced, &text_new)
        }, 
        Op::SP => {
            rename_proc::sp(targets)
        }, 
        Op::SP2 => {
            rename_proc::sp2(targets)
        }, 
        _ => {
            panic!("Unknown operation");
        }
    };

    let answer = confirm(results.as_ref());
    if answer {
        execute(&config.path, results.as_ref())?;
    }

    Ok(())
}

fn parse_args() -> Config {
    let mut args = env::args();
    // skip the first argument which is the path of exe
    args.next();

    let mut arg = args.next().expect("[Lack of inputs] path(directory or file), operation, parameter(s)");
    let mut path = PathBuf::from(arg);
    if !path.exists() {
        panic!("[Wrong input] Directory or File does not exists: {:?}", path);
    }
    // transform to directory if input is a filename
    if path.is_file() {
        path = path.parent().expect("[Unexpected] Failed to access directory").to_path_buf();
    }

    arg = args.next().expect("[Lack of inputs] operation, parameter(s)");
    let op = match arg.as_str() {
        "insert" => { 
            let text = args.next().expect("[Lack of inputs] (1) Text to be inserted");
            Op::Insert(String::from(text))
         }, 
        "insert_tail" => { 
            let text = args.next().expect("[Lack of inputs] (1) Text to be inserted");
            Op::InsertTail(String::from(text))
         }, 
        "remove" => {
            let n = match args.next() {
                Some(arg) => { 
                    arg.parse::<usize>().expect(&format!("[Wrong input] Failed to parse \"{}\" to positive integer", arg))
                }, 
                None => {
                    println!("[Lack of inputs] (1) Number of characters to be removed");
                    let arg = prompt();
                    arg.parse::<usize>().expect(&format!("[Wrong input] Failed to parse \"{}\" to positive integer", arg))
                }
            };
            Op::Remove(n)
        }, 
        "remove_tail" => {
            let n = match args.next() {
                Some(arg) => { 
                    arg.parse::<usize>().expect(&format!("[Wrong input] Failed to parse \"{}\" to positive integer", arg))
                }, 
                None => {
                    println!("[Lack of inputs] (1) Number of characters to be removed");
                    let arg = prompt();
                    arg.parse::<usize>().expect(&format!("[Wrong input] Failed to parse \"{}\" to positive integer", arg))
                }
            };
            Op::RemoveTail(n)
        }
        "replace" => {
            let p1 = args.next().expect("[Lack of inputs] (1) Text to be replaced, (2) New Text");
            let p2 = args.next().expect("[Lack of inputs] (1) New Text");
            Op::Replace(p1, p2)
        }, 
        "sp" => {
            Op::SP
        }, 
        "sp2" => {
            Op::SP2
        }, 
        _ => {
            Op::Unknown
        }
    };

    Config {
        path: path, 
        op: op
    }
}

#[derive(Debug)]
struct Config {
    path: PathBuf, 
    op: Op, 
}


use pkr_rename::filename;

pub fn list_files(path: &PathBuf) -> Box<Vec<filename::Filename>> {
    assert!(path.exists() && path.is_dir());

    let targets = path.read_dir().expect("The process ceased due to lack of permissons to access the directory (or maybe other causes)")
    .map(|res| {
        // an alternative way is to ignore err cases by call Result::map
        // https://doc.rust-lang.org/std/fs/struct.ReadDir.html
        res.expect("intermittent IO error")
    })
    .filter(|dir_entry| {
        dir_entry.metadata().expect("??").is_file()
    })
    .map(|file| {
        let file = file.path();
        filename::Filename::new(
            file.file_stem().expect("no file name").to_os_string().into_string().expect("Non-Utf-8 string are detected"), 
            match file.extension() {
                Some(ext) => Some( ext.to_os_string().into_string().expect("Non-Utf-8 string are detected") ), 
                None => None
            }
        )
    })
    .collect::<Vec<filename::Filename>>();

    Box::<Vec<filename::Filename>>::new(targets)
}


fn confirm(results: &Vec<filename::Res>) -> bool {
    println!("Results after renaming: ");

    for (idx, result) in results.iter().enumerate() {
        println!("[{}] \n{}  ->\n{}", idx, result.orig_with_highlight(), result.alter_with_highlight());
    }

    println!("\nConfirm (y/N)");
    if prompt() == "y" { true } else { false }
}

fn prompt() -> String {
    use std::io;
    let mut answer = String::new();
    let _n = io::stdin().read_line(&mut answer).unwrap();
    // println!("{} bytes: {:?}", _n, answer.as_bytes());

    // strip off line-break
    String::from(answer.trim_end())
}

fn execute(root: &PathBuf, results: &Vec<filename::Res>) -> std::io::Result<()> {

    let dir = root.as_path().to_str().expect("Non-Utf-8 string are detected");
    for res in results.iter() {
        fs::rename(
            format!("{}\\{}", dir, res.orig()), 
            format!("{}\\{}", dir, res.alter())
        )?;
    }
    Ok(())
}



pub fn print_type<T: ?Sized>(_: &T) {
    println!("{}", type_name::<T>());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_not_exists(){
        
    }

    // not real test
    #[test]
    fn test_char() {
        let chars: Vec<char> = "予定調和".chars().collect();
        let ass = vec!('予', '定', '調', '和');
        assert_eq!(ass, chars);
    }

    #[test]
    fn general_test() {
        use std::path::PathBuf;

        let path = PathBuf::from("D:\\");
        let targets: Box<Vec<Filename>> = list_files(&path);
        // println!("{:?}", targets);
        rename_proc::sp2(targets);
    
        // let answer = confirm(targets.as_ref(), results.as_ref());
    }
    
}


// Rust's move is different with C++'s move
// https://stackoverflow.com/questions/3106110/what-is-move-semantics
// https://stackoverflow.com/questions/29490670/how-does-rust-provide-move-semantics

// https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html
// the only difference is whether you are allowed to access x after the assignment. 
// Under the hood, both a copy and a move can result in bits being copied in memory, 
// although this is sometimes optimized away.