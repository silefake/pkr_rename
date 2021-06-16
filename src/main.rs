use std::env;
use std::any::type_name;
use std::path::PathBuf;
use std::fs;
use regex::Regex;


fn main() -> std::io::Result<()> {
    let config = parse_args();
    println!("{:?}", config);

    let targets = list_files(&config.path);
    // println!("{:?}", targets);

    let mut results = Vec::<String>::new();
    match config.op_type {
        Op::INSERT(text) => {
            results = op_insert_process(&targets, &text);
        }, 
        Op::REMOVE(n) => {
            results = op_remove_process(&targets, n);
        }, 
        Op::REPLACE(text_replaced, text_new) => {
            results = op_replace_process(&targets, &text_replaced, &text_new);
        }, 
        _ => {
            panic!("Unknown operation");
        }
    }

    let answer = op_prompt(&targets, &results);
    if answer {
        op_execute(&config.path, &targets, &results)?;
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
    let op_type = match arg.as_str() {
        "insert" => { 
            let text = args.next().expect("[Lack of inputs] (1) Text to be inserted");
            Op::INSERT(String::from(text))
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
            Op::REMOVE(n)
        }, 
        "replace" => {
            let p1 = args.next().expect("[Lack of inputs] (1) Text to be replaced, (2) New Text");
            let p2 = args.next().expect("[Lack of inputs] (1) New Text");
            Op::REPLACE(p1, p2)
        }, 
        _ => {
            Op::UNKNOWN
        }
    };

    Config {
        path: path, 
        op_type: op_type
    }
}

#[derive(Debug)]
struct Config {
    path: PathBuf, 
    op_type: Op, 
}

#[derive(Debug)]
enum Op {
    INSERT(String), // ADD
    REMOVE(usize), 
    REPLACE(String, String), 
    UNKNOWN, 
}

fn list_files(path: &PathBuf) -> Vec<String> {
    path.read_dir().unwrap()
    .map(|res| {
        // an alternative way is to ignore err cases by call Result::map
        res.unwrap()
    })
    .filter(|dir_entry| {
        dir_entry.metadata().unwrap().is_file()
    })
    .map(|file| {
        file.file_name().into_string().unwrap()
    })
    .collect::<Vec<String>>()
}

fn op_insert_process(targets: &Vec<String>, object: &str) -> Vec<String> {
    targets.iter()
    .map(|filename| {
        format!("{}{}", object, filename)
    })
    .collect::<Vec<String>>()
}

fn op_prompt(targets: &Vec<String>, results: &Vec<String>) -> bool {
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

fn op_execute(root: &PathBuf, targets: &Vec<String>, results: &Vec<String>) -> std::io::Result<()> {
    assert_eq!(targets.len(), results.len());

    let dir = root.as_path().to_str().unwrap();
    for (idx, filename) in targets.iter().enumerate() {
        fs::rename(
            format!("{}\\{}", dir, &filename), 
            format!("{}\\{}", dir, &results[idx]
        ))?;
    }
    Ok(())
}

fn op_remove_process(targets: &Vec<String>, n: usize) -> Vec<String> {
    use std::iter::FromIterator;
    // TODO: limit maximum # of characters that can be remove

    targets.iter()
    .map(|filename| {
        let chars: Vec<char> = filename.chars().collect();
        // println!("{:?}", &chars[n..]);

        String::from_iter(chars[n..].iter())
    })
    .collect::<Vec<String>>()
}

// TODO: Now we use Escape to match literally, maybe add an option to determine if Regex syntax will be used
fn op_replace_process(targets: &Vec<String>, replaced: &str, object: &str) -> Vec<String> {
    let re = Regex::new(&regex::escape(replaced)).unwrap();

    targets.iter()
    .map(|filename| {
        if let Some(mat) = re.find(filename) {
            let head_part = &filename[..mat.start()];
            let tail_part = &filename[mat.end()..];

            format!("{}{}{}", head_part, object, tail_part)
        }
        else {
            format!("{}", filename)
        }
    })
    .collect::<Vec<String>>()
}





pub fn print_type<T: ?Sized>(_: &T) {
    println!("{}", type_name::<T>());
}


#[cfg(test)]
mod tests {
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
    
}


