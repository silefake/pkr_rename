
use regex::Regex;

use crate::util;
use crate::filename;

#[derive(Debug)]
pub enum Op {
    Insert(String), // ADD
    InsertTail(String), 
    Remove(usize), 
    RemoveTail(usize), 
    Replace(String, String), 
    SP, 
    SP2, 
    Unknown, 
}

pub fn insert(targets: Box<Vec<filename::Filename>>, object: &str) -> Box<Vec<filename::Res>> {
  let results = targets.into_iter()
  .map(|name| {
      filename::Res::new(
          name.clone(), 
          format!("{}{}", object, name.stem()), 
          (0, 0), 
          (0, object.len())
      )
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}

pub fn insert_tail(targets: Box<Vec<filename::Filename>>, object: &str) -> Box<Vec<filename::Res>> {
  let results = targets.into_iter()
  .map(|name| {
      filename::Res::new(
          name.clone(), 
          format!("{}{}", name.stem(), object), 
          (0, 0), 
          (name.len(), name.len() + object.len())
      )
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}

pub fn remove(targets: Box<Vec<filename::Filename>>, n: usize) -> Box<Vec<filename::Res>> {
  // TODO: limit maximum # of characters that can be remove

  let results = targets.into_iter()
  .map(|name| {

      if let Some((idx, _ch)) = name.stem().char_indices().nth(n) {
          filename::Res::new(
              name.clone(), 
              format!("{}", &name.stem()[idx..]), 
              (0, idx), 
              (0, 0)
          )
      } else {
          panic!("rename_proc::remove(): There is a file's name too short")
      }
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}

pub fn remove_tail(targets: Box<Vec<filename::Filename>>, n: usize) -> Box<Vec<filename::Res>> {
  // TODO: limit maximum # of characters that can be remove

  let results = targets.into_iter()
  .map(|name| {
      let length = util::stc(&name.stem()).len();
      if let Some((idx, _ch)) = name.char_indices().nth(length - n) {
          filename::Res::new(
              name.clone(), 
              format!("{}", &name.stem()[..idx]), 
              (idx, name.stem().len()), 
              (0, 0)
          )
      } else {
          panic!("rename_proc::remove(): There is a file's name too short")
      }
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}

// TODO: Now we use Escape to match literally, maybe add an option to determine if Regex syntax will be used
pub fn replace(targets: Box<Vec<filename::Filename>>, pattern: &str, object: &str) -> Box<Vec<filename::Res>> {
  // let re = Regex::new(&regex::escape(pattern)).unwrap();
  let re = Regex::new(pattern).unwrap();

  let results = targets.into_iter()
  .filter_map(|name| {
      if let Some(mat) = re.find(&name) {
          let head_part = &name.stem()[..mat.start()];
          let tail_part = &name.stem()[mat.end()..];  

          Some(filename::Res::new(
              name.clone(), 
              format!("{}{}{}", head_part, object, tail_part), 
              (mat.start(), mat.end()), 
              (mat.start(), mat.start() + object.len())
          ))
      }
      else {
          None
      }
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}


pub fn sp(targets: Box<Vec<filename::Filename>>) -> Box<Vec<filename::Res>> {
  let re = Regex::new("20[0-9]{2}年[0-9]+月[0-9]+日[0-9_]*").unwrap();

  let split_time = |name: &str| -> (String, String, String, usize, usize) {
      if let Some(mat) = re.find(name) {
          let author = name[..mat.start()].to_string();
          let time = name[mat.start()..mat.end()].to_string();
          let comment = name[mat.end()..].to_string();

          (author, time, comment, mat.start(), mat.end())
      } else {
          println!("{}", name);
          panic!();
      }
  };

  fn reformat_time(residual: &str) -> String {
      fn padding_number_2(num: &str) -> String {
          assert!(num.len() > 0 && num.len() <= 2);

          if num.len() == 1 {
              ["0", num].join("")
          } else {
              num.to_string()
          }
      }
      let (year, residual) = residual.split_once("年").unwrap();
      let (month, residual) = residual.split_once("月").unwrap();
      let (day, residual) = residual.split_once("日").unwrap();
      let (hour, minute) = if let Some((hour, minute)) = residual.split_once("_"){
          (padding_number_2(hour), minute.to_string())
      } else {
          ("".to_string(), "".to_string())
      };

      let month = padding_number_2(month);
      let day = padding_number_2(day);

      if hour == "" {
          [year, &month, &day].join("")
      } else {
          [year, &month, &day, "_", &hour, &minute].join("")
      }
  }

  let results = targets.into_iter()
  .map(|name| {
      let (author, time, comment, tr1, tr2) = split_time(name.stem());
      let time_reformat = reformat_time(&time);
      
      filename::Res::new(
          name.clone(), 
          format!("{}{}{}", author, time_reformat, comment), 
          (tr1, tr2), 
          (tr1, tr1 + time_reformat.len())
      )
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}

pub fn sp2(targets: Box<Vec<filename::Filename>>) -> Box<Vec<filename::Res>> {
  let re = Regex::new(r"20[12][0-9]-\d{2}-\d{2}_*").unwrap();

  let results = targets.into_iter()
  .filter_map(|name| {

    if let Some(mat) = re.find(name.stem()) {
      let t1 = name.stem()[0..mat.start()].to_string();
      let t2 = name.stem()[mat.start()..mat.end()].to_string();
      let t3 = name.stem()[mat.end()..].to_string();

      let t2 = format!("{}{}{}_ss", &t2[0..4], &t2[5..7], &t2[8..10]);

      Some(filename::Res::new(
        name.clone(), 
        format!("{}{}{}", t1, &t2, t3), 
        (mat.start(), mat.end()), 
        (mat.start(), mat.start() + 11)
      ))
    } else {
      None
    }
  })
  .collect::<Vec<filename::Res>>();

  Box::new(results)
}