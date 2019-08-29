use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum Tag {
    Button,
    Header(u8),
    None,
}


#[derive(Debug)]
pub struct Lang {
    pub code: String,    // e.g. "en" or "da"
    pub content: String, // the text
    pub tag: Tag,
}


#[derive(Debug)]
pub struct Snippet {
    pub dirs: Vec<Lang>,
}

impl Snippet {
    pub fn new() -> Self {
        Self {
            dirs: Vec::new(),
        }
    }

    pub fn add_lang(&mut self, code: String, md: &str) {
        let mut index = 0;
        let chars = md.chars().collect::<Vec<char>>();

        let mut content = String::new();
        let mut tag = Tag::None;

        'outer: while index < chars.len() {
            let c = chars[index];

            match c {
                '[' => {
                    tag = Tag::Button;
                    index += 1;

                    while chars[index] != ']' {
                        content.push(chars[index]);
                        index += 1
                    }

                    break
                },

                '#' => {
                    let mut header_size = 1;
                    index += 1;

                    while chars[index] == '#' {
                        header_size += 1;
                        index += 1
                    }

                    tag = Tag::Header(header_size);
                },

                _ => {
                    content.push(c)
                }
            }

            index += 1
        }

        self.dirs.push(
            Lang {
                code: code.to_owned(),
                content,
                tag,
            }
        )
    }
}



pub type LangFile = (String, String);


pub struct Page {
    pub content: Vec<Snippet>,
    pub path: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            path: String::new(),
        }
    }

    pub fn add_files(&mut self, file_groups: Vec<Vec<LangFile>>) {
        for file_group in file_groups {


        }
    }
}
