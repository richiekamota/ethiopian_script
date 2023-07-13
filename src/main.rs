use std::fs::metadata;
use std::fs::File;
use std::env::args;
use std::time::Instant;
use std::io::{Write,BufRead,BufReader,BufWriter};
use regex::Regex;
use regex::Captures;

fn main(){
  let args: Vec<String> = args().collect();
  if args.len() !=3 {
    eprintln!("Usage: `source` `target`");
    return;
  }

  let source = &args[1];
  let target = &args[2];
 
  let file = File::open(source).unwrap();
  let input = BufReader::new(file);
  let output = File::create(target).unwrap();
  let mut writer = BufWriter::new(output);
  let start = Instant::now();
  let regex = Regex::new(r"Ethiopia(ns?|n)?").unwrap();
    for line in input.lines(){
      let line = line.unwrap();
      let replaced_line = regex.replace_all(&line, |caps: &Captures|
      if caps[0].ends_with('n') {
          return "Zimbabwean".to_owned();
      } else if caps[0].ends_with('a'){
          return "Zimbabwe".to_owned();
      } else {
          return "Zimbabweans".to_owned();
      }
    );
    writeln!(writer,"{}",replaced_line).unwrap();
  }

  writer.flush().unwrap();
  let source_len = metadata(source).unwrap().len();
  let target_len = metadata(target).unwrap().len();

  println!("Source len: {} bytes", source_len);
  println!("Target len: {} bytes", target_len);
  println!("Time elapsed: {:?}", start.elapsed());
  println!("Replacement completed...............");

}