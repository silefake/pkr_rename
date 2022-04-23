
#[derive(Clone)]
pub struct Filename {
  stem: String, 
  extension: Option<String>, 
  full: String
}

impl Filename {
  pub fn new(stem: String, extension: Option<String>) -> Filename {
    let full = format!("{}.{}", &stem, extension.as_ref().unwrap_or(&"".to_string()));
    Filename {
      stem, 
      extension, 
      full
    }
  }

  pub fn stem(&self) -> &String {
    &self.stem
  }

  pub fn extension(&self) -> String {
    self.extension.as_ref().unwrap_or(&"".to_string()).clone()
  }
}

impl std::ops::Deref for Filename {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.full
  }
}

impl std::fmt::Display for Filename {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", *self)
  }
}


use crate::util;

pub struct Res {
  orig: Filename, 
  alter: Filename, 
  // trimming range
  _tr: (usize, usize), 
  // inserting range, 
  _ir: (usize, usize)
}

impl Res {
  pub fn new(orig: Filename, alter_stem: String, _tr: (usize, usize), _ir: (usize, usize)) -> Res {
    if !(_tr.0 <= _tr.1 && _tr.1 <= orig.len()) {
      panic!("Res::new(): Wrong mutation range");
    }
    let ext = orig.extension();

    Res {
      orig, 
      alter: Filename::new(alter_stem, Some(ext)), 
      _tr, 
      _ir 
    }
  }

  pub fn orig(&self) -> String {
    format!("{}.{}", self.orig.stem(), self.orig.extension())
  }

  pub fn orig_with_highlight(&self) -> String {
    format!(
      "{}\x1b[44m{}\x1b[0m{}", 
      &self.orig[0..self._tr.0], 
      &self.orig[self._tr.0..self._tr.1], 
      &self.orig[self._tr.1..]
    )
  }

  pub fn alter(&self) -> String {
    format!("{}.{}", self.alter.stem(), self.alter.extension())
  }

  pub fn alter_with_highlight(&self) -> String {
    format!(
      "{}\x1b[41m{}\x1b[0m{}", 
      &self.alter[0..self._ir.0], 
      &self.alter[self._ir.0..self._ir.1], 
      &self.alter[self._ir.1..]
    )
  }


}