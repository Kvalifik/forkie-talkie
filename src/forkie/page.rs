use std::collections::HashMap;
use std::path::Path;

use std::fmt;

#[derive(Debug)]
pub enum Tag {
    Button,
    Header(u8),
    None,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Tag::Button => write!(f, "v_btn"),
            &Tag::Header(a) => write!(f, "h{}", a),
            _ => write!(f, "p"),
        }
    }
}

#[derive(Debug)]
pub struct Lang {
    pub content: String, // the text
    pub tag: Tag,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}


#[derive(Debug)]
pub struct Snippet {
    pub code: String,
    pub dirs: Vec<Lang>,
}

impl Snippet {
    pub fn new(code: String) -> Self {
        Self {
            dirs: Vec::new(),
            code,
        }
    }

    pub fn add_lang(&mut self, md: &str) {
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
                content,
                tag,
            }
        )
    }
}



pub type LangFile = (String, String);


pub struct Page {
    pub content: Vec<Snippet>,
    pub names: Vec<String>,
    pub path: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            names: Vec::new(),
            path: String::new(),
        }
    }

    pub fn add_files(&mut self, file_groups: Vec<Vec<LangFile>>) {
        for file_group in file_groups {
            let mut splits: Vec<Vec<&str>> = Vec::new();

            for file in file_group.iter() {
                splits.push(file.1.split("---").collect::<Vec<&str>>())
            }

            for (j, split) in splits.iter().enumerate() {
                let mut snippet = Snippet::new(file_group[j].0.clone());

                for section in split {
                    snippet.add_lang(section);

                    if j == 0 {
                        self.names.push(snippet.dirs.last().unwrap().content.split(" ").collect::<Vec<&str>>()[0].to_owned())
                    }
                }

                self.content.push(snippet)
            }
        }
    }

    pub fn as_i18_yaml(&self) -> String {
        let mut result = String::new();

        for snippet in self.content.iter() {
            result.push_str(&format!("<i18n locale=\"{}\" lang=\"yaml\">\n", snippet.code));

            for (i, lang) in snippet.dirs.iter().enumerate() {
                result.push_str(&format!("\t__{}_{}__{}: \"{}\"", self.names[i].trim(), i, lang.tag, lang.content.trim()));
                result.push('\n')
            }

            result.push_str("</i18n>\n\n");
        }

        result
    }
}
