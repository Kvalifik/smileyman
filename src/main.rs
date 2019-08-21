use std::fs;
use std::fs::File;
use std::fs::metadata;

use std::env;

use std::io::prelude::*;
use std::path::Path;

fn write(path: &str, data: &str) {
  let path = Path::new(path);

  let split_name = path.file_name().unwrap().to_str().unwrap().split('.');
  let split: Vec<&str> = split_name.collect();

  let path_split = path.to_str().unwrap().split('/').collect::<Vec<&str>>();

  let path_real  = if path_split.len() > 1 {
    format!("{}/{}.md", path_split[0 .. path_split.len() - 1].join("/"), split[0])
  } else {
    format!("{}.md", split[0])
  };

  let mut output_file = File::create(&path_real).unwrap();
  match output_file.write_all(data.as_bytes()) {
    Ok(_)    => (),
    Err(why) => println!("{}", why)
  }
}

fn grap_path(path: &str) {
    let meta = match metadata(path) {
        Ok(data) => data,
        Err(why) => panic!("{}", why),
    };

    if meta.is_file() {
        let split: Vec<&str> = path.split('.').collect();

        if *split.last().unwrap() == "vue" {
            write(path, &convert_text(&file_content(path)))
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        for folder_path in paths {
            let folder_path = format!("{}", folder_path.unwrap().path().display());
            let split: Vec<&str> = folder_path.split('.').collect();

            if Path::new(&folder_path).is_dir() || *split.last().unwrap() == "vue" {
                grap_path(&folder_path)
            }
        }
    }
}

fn file_content(path: &str) -> String {
    let display = Path::new(path).display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("Failed to read {}: {}", display, why),
        Ok(_)    => s.to_owned(),
    }
}

fn convert_text(content: &str) -> String {
    let mut converted = String::new();

    'outer: for line in content.lines() {
        let mut started = false;
        let mut index = 0;
        let chars: Vec<char> = line.chars().collect();

        let mut result = String::new();

        while index < chars.len() {
            let tags: &'static [&'static str] = &["h1", "h2", "h3", "h4", "h5", "p", "b", "i", "strong", "em", "v-btn", "button"];

            for tag in tags.iter() {
                if peek_range(&chars, index, tag.len() + 1) == format!("<{}", tag) {
                    started = true;

                    index += tag.len() + 1;

                    if index < chars.len() && chars[index] != '>' {
                        let mut j = index;

                        while j < chars.len() {
                            j     += 1;
                            index += 1;

                            if j < chars.len() && chars[j] == '>' {
                                index += 1;

                                break
                            }
                        }
                    } else {
                        index += 1
                    }

                    break

                } else if peek_range(&chars, index, tag.len() + 3) == format!("</{}>", tag) {
                    started = false;

                    let mut prefix = String::new();

                    if tag.contains('h') {
                        let acc = match *tag {
                            "h1" => 1,
                            "h2" => 2,
                            "h3" => 3,
                            "h4" => 4,
                            "h5" => 5,
                            _ => unreachable!()
                        };

                        for _ in 0 .. acc {
                            prefix.push('#')
                        }

                        prefix.push(' ')
                    }

                    converted.push_str(&format!("{}{}\n", prefix, result));
                    index += tag.len() + 3;

                    result = String::new();

                    break
                }
            }

            if index < chars.len() {
                let c = &chars[index];

                if started {
                    result.push(*c);
                }

                index += 1
            }
        }
    }

    converted
}

fn peek_range(content: &Vec<char>, index: usize, len: usize) -> String {
    if index + len - 1 >= content.len() {
        return String::new()
    } else {
        content[index .. index + len].iter().collect::<String>()
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        grap_path(&args[1])
    }
}
