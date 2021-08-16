use std::env;
use std::any::type_name;
use std::path::PathBuf;
use std::fs;

use regex::Regex;


fn main() -> std::io::Result<()> {
    let config = parse_args();
    println!("{:?}", config);

    let targets: Box<Vec<Filename>> = list_files(&config.path);
    // println!("{:?}", targets);

    let results = match config.op {
        Op::Insert(text) => {
            op_insert(targets.as_ref(), &text)
        },
        Op::InsertTail(text) => {
            op_insert_tail(targets.as_ref(), &text)
        }, 
        Op::Remove(n) => {
            op_remove(targets.as_ref(), n)
        }, 
        Op::RemoveTail(n) => {
            op_remove_tail(targets.as_ref(), n)
        }
        Op::Replace(text_replaced, text_new) => {
            op_replace(targets.as_ref(), &text_replaced, &text_new)
        }, 
        _ => {
            panic!("Unknown operation");
        }
    };

    let answer = op_prompt(targets.as_ref(), results.as_ref());
    if answer {
        op_execute(&config.path, targets.as_ref(), results.as_ref())?;
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

#[derive(Debug)]
enum Op {
    Insert(String), // ADD
    InsertTail(String), 
    Remove(usize), 
    RemoveTail(usize), 
    Replace(String, String), 
    Unknown, 
}

pub struct Filename {
    stem: String, 
    extension: Option<String>
}

impl std::ops::Deref for Filename {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.stem
    }
}
impl std::fmt::Display for Filename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.extension {
            Some(ext) => {
                write!(f, "{}.{}", self.stem, ext)
            }, 
            None => {
                write!(f, "{}", self.stem)
            }
        }
    }
}

impl Filename {
    pub fn stem(&self) -> String {
        String::from(&self.stem)
    }
    pub fn extension(&self) -> String {
        String::from(self.extension.as_ref().unwrap_or(&"".to_string()))
    }
}

pub fn list_files(path: &PathBuf) -> Box<Vec<Filename>> {
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
        Filename {
            stem: file.file_stem().expect("no file name").to_os_string().into_string().expect("Non-Utf-8 string are detected"),
            extension: match file.extension() {
                Some(ext) => Some( ext.to_os_string().into_string().expect("Non-Utf-8 string are detected") ), 
                None => None
            }
        }
    })
    .collect::<Vec<Filename>>();

    Box::<Vec<Filename>>::new(targets)
}

fn op_insert(targets: &Vec<Filename>, object: &str) -> Box<Vec<String>> {
    let results = targets.iter()
    .map(|filename| {
        format!("{}{}", object, filename)
    })
    .collect::<Vec<String>>();

    Box::new(results)
}

fn op_insert_tail(targets: &Vec<Filename>, object: &str) -> Box<Vec<String>> {
    let results = targets.iter()
    .map(|filename| {
        format!("{}{}.{}", filename.stem(), object, filename.extension())
    })
    .collect::<Vec<String>>();

    Box::new(results)
}

fn op_prompt(targets: &Vec<Filename>, results: &Vec<String>) -> bool {
    assert_eq!(targets.len(), results.len());

    println!("Results after renaming: ");

    for (idx, filename) in targets.iter().enumerate() {
        println!("[{}] {}  ->  {}", idx, filename, results[idx]);
    }

    println!("\nConfirm (y/N)");

    let answer = prompt();
    if answer == "y" { true } else { false }
}

fn prompt() -> String {
    use std::io;
    let mut answer = String::new();
    let _n = io::stdin().read_line(&mut answer).unwrap();
    // println!("{} bytes: {:?}", _n, answer.as_bytes());

    // strip off line-break
    String::from(answer.trim_end())
}

fn op_execute(root: &PathBuf, targets: &Vec<Filename>, results: &Vec<String>) -> std::io::Result<()> {
    assert_eq!(targets.len(), results.len());

    let dir = root.as_path().to_str().expect("Non-Utf-8 string are detected");
    for (idx, filename) in targets.iter().enumerate() {
        fs::rename(
            format!("{}\\{}", dir, &filename), 
            format!("{}\\{}", dir, &results[idx]
        ))?;
    }
    Ok(())
}

fn op_remove(targets: &Vec<Filename>, n: usize) -> Box<Vec<String>> {
    use std::iter::FromIterator;
    // TODO: limit maximum # of characters that can be remove

    let results = targets.iter()
    .map(|filename| {
        let chars: Vec<char> = filename.chars().collect();
        // println!("{:?}", &chars[n..]);

        format!("{}.{}", String::from_iter(chars[n..].iter()), filename.extension() )
    })
    .collect::<Vec<String>>();

    Box::new(results)
}

fn op_remove_tail(targets: &Vec<Filename>, n: usize) -> Box<Vec<String>> {
    use std::iter::FromIterator;
    // TODO: limit maximum # of characters that can be remove

    let results = targets.iter()
    .map(|filename| {
        let chars: Vec<char> = filename.chars().collect();
        let length = chars.len();
        // println!("{:?}", &chars[n..]);

        format!("{}.{}", String::from_iter(chars[..(length - n)].iter()), filename.extension() )
    })
    .collect::<Vec<String>>();

    Box::new(results)
}

// TODO: Now we use Escape to match literally, maybe add an option to determine if Regex syntax will be used
fn op_replace(targets: &Vec<Filename>, replaced: &str, object: &str) -> Box<Vec<String>> {
    let re = Regex::new(&regex::escape(replaced)).unwrap();
    // let re = Regex::new(replaced).unwrap();

    let results = targets.iter()
    .map(|filename| {
        if let Some(mat) = re.find(filename) {
            let head_part = &filename[..mat.start()];
            let tail_part = &filename[mat.end()..];

            format!("{}{}{}.{}", head_part, object, tail_part, filename.extension() )
        }
        else {
            format!("{}", filename)
        }
    })
    .collect::<Vec<String>>();

    Box::new(results)
}





pub fn print_type<T: ?Sized>(_: &T) {
    println!("{}", type_name::<T>());
}


#[cfg(test)]
mod tests {
    // #![feature(test)]
    // extern crate test;
    // use test::Bencher;
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

    // #[bench]
    // fn bench_list_file(b: &mut Bencher) {
    //     b.iter(|| {
    //         let _targets = list_files(&PathBuf::from("D:\\Steam"));
    //     });
    // }
    
}


// Rust's move is different with C++'s move
// https://stackoverflow.com/questions/3106110/what-is-move-semantics
// https://stackoverflow.com/questions/29490670/how-does-rust-provide-move-semantics

// https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html
// the only difference is whether you are allowed to access x after the assignment. 
// Under the hood, both a copy and a move can result in bits being copied in memory, 
// although this is sometimes optimized away.