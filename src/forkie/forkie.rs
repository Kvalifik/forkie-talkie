use std::fs;
use std::fs::File;
use std::fs::metadata;

use std::env;

use std::io::prelude::*;
use std::path::Path;

use super::page::Page;

pub fn grab_path(path: &str) {
    let meta = match metadata(path) {
        Ok(data) => data,
        Err(why) => panic!("{}", why),
    };

    if meta.is_file() {
        let split: Vec<&str> = path.split('.').collect();

        if *split.last().unwrap() == "vue" {
            // write(path, &internationalize(&file_content(path)))
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        for folder_path in paths {
            let folder_path = format!("{}", folder_path.unwrap().path().display());
            let split: Vec<&str> = folder_path.split('.').collect();

            if Path::new(&folder_path).is_dir() || *split.last().unwrap() == "vue" {
                grab_path(&folder_path)
            }
        }
    }
}

pub fn file_content(path: &str) -> String {
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

fn peek_range(content: &Vec<char>, index: usize, len: usize) -> String {
    if index + len - 1 >= content.len() {
        return String::new()
    } else {
        content[index .. index + len].iter().collect::<String>()
    }
}

pub fn internationalize(content: &mut String, page: &Page) {
    let mut index = 0;
    let chars: Vec<char> = content.chars().collect();

    let mut result = String::new();
    let mut page_index = 0;
    let mut start = 0;


    while index < chars.len() {
        let tags: &'static [&'static str] = &[
            "h1", "h2", "h3", "h4", "h5", "p", "b", "i", "strong", "em", "v-btn", "button", "div", "span", "v-card-title",
            "vue-markdown", "v-card-text",
        ];

        for tag in tags.iter() {
            if peek_range(&chars, index, tag.len() + 1) == format!("<{}", tag) {
                index += tag.len() + 1;

                if index < chars.len() && chars[index] != '>' {
                    let mut j = index;

                    while j < chars.len() {
                        j += 1;

                        if j < chars.len() && chars[j] == '>' {

                            index += j - index;
                            break
                        }
                    }
                }

                start = index + 1;

                break

            } else if peek_range(&chars, index, tag.len() + 3) == format!("</{}>", tag) {
                // recalculate start, wups
                let offset = (chars.len() as i32 - content.len() as i32).abs() as usize;

                let new_index = if chars.len() > content.len() {
                    start -= offset;
                    index - offset
                } else {
                    start += offset;
                    index + offset
                };

                content.replace_range(start .. new_index, &format!("{{{{ $t('__{}_{}__{}') }}}}", page.names[page_index].trim(), page_index, tag.replace('-', "_")));
                index += tag.len() + 3;

                page_index += 1;

                break
            }
        }

        index += 1
    }
}