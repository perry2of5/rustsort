use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;
use std::str::Chars;

fn main() {
    // println!("Hello, world!");

    let cmd_line_arg_iter = env::args_os().skip(1);
    let mut base_node : Node = Node::new('\r');
    for argument in cmd_line_arg_iter {
        // println!("{:?}", argument);
        let path = Path::new(&argument);
        match path.exists() {
            false => println!("file {:?} does not exist",  path),
            true => add_file_to_sort_data(&path, & mut base_node)
        }
    }
    base_node.dump();
}

fn add_file_to_sort_data(path: &Path, base_node: & mut Node) {
  match File::open(path) {
    Ok(file_handle) => {
      for line in BufReader::new(file_handle).lines() {
        let line_string = line.unwrap();
        let original_string = line_string.clone();
        base_node.add_string(& mut line_string.chars(), original_string);
      }
    },
    Err(err) => println!("{}", err)
  }
}


pub struct Node {
  character : char,
  terminations: usize,
  nexts: Vec<Node>,
  original_strings: Vec<String>
}

impl Node {
  pub fn new(character : char) -> Node {
    // println!("Node::new({})", character);
    Node {
      character: character,
      terminations: 0,
      nexts: Vec::new(),
      original_strings: Vec::new()
    }
  }

  pub fn add_string(&mut self, chars_iterator : & mut Chars, original_string: String) -> &mut Node {

    match chars_iterator.next() {
        Some(character) => {
          // TODO, find x if already in list, add new node if not, follow path otherwise.
          let mut new_node = self.find_or_create_node(character);
          new_node.add_string(chars_iterator, original_string);
          // self.nexts.push(new_node)
        },
        None => {
          self.terminations = self.terminations + 1;
          self.original_strings.push(original_string);
        }
    }
    self
  }

  fn find_or_create_node(&mut self, character: char) -> & mut Node {
    let initial_length = self.nexts.len();
    for i in 0 .. initial_length {
      if self.nexts[i].character == character {
        return & mut self.nexts[i]
      } else if self.nexts[i].character > character {
        let new_node = Node::new(character);
        self.nexts.insert(i, new_node);
        return & mut self.nexts[i]
      }
    }

    let new_node = Node::new(character);
    self.nexts.push(new_node);
    return & mut self.nexts[initial_length]
  }

  pub fn dump(&self) {
    for _ in 0 .. self.terminations {
      println!("");
    }

    for ref sub_node in self.nexts.iter() {
      sub_node.dump_sub_strings();
    }
  }

  fn dump_sub_strings(&self) {
    for str in self.original_strings.iter() {
      println!("{}", str);
    }

    for ref sub_node in self.nexts.iter() {
      sub_node.dump_sub_strings();
    }
  }
}


