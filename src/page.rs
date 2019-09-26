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
            &Tag::Button => write!(f, "button"),
            &Tag::Header(a) => write!(f, "title_h{}", a),
            _ => write!(f, "text_p"),
        }
    }
}

#[derive(Debug)]
pub struct Lang {
    pub content: String, // the text
    pub tag: Tag,
}

impl Lang {
    pub fn to_html(&self) -> String {
        let mut new_content = self.content.clone();

        while new_content.find("**").is_some() {
            new_content = new_content.replacen("**", "<b>", 1);
            new_content = new_content.replacen("**", "</b>", 1);
        }

        while new_content.find("*").is_some() {
            new_content = new_content.replacen("*", "<i>", 1);
            new_content = new_content.replacen("*", "</i>", 1);
        }

        new_content
    }

    pub fn to_trim(&self) -> String {
        let mut result = self.content.clone();

        result.retain(|x| !['*', '#'].contains(&x));
        result = result.replace('+', "\\+");

        result
    }
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

                    break;
                }

                '#' => {
                    let mut header_size = 1;
                    index += 1;

                    while chars[index] == '#' {
                        header_size += 1;
                        index += 1
                    }

                    tag = Tag::Header(header_size);
                }

                _ => content.push(c),
            }

            index += 1
        }

        self.dirs.push(Lang { content, tag })
    }
}

pub type LangFile = (String, String);

pub struct Page {
    pub content: Vec<Snippet>,
    pub names: Vec<String>,
    pub path: String,
}

impl Page {
    pub fn new(path: String) -> Self {
        Self {
            content: Vec::new(),
            names: Vec::new(),
            path,
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

                for (i, section) in split.iter().enumerate() {
                    if i == split.len() - 1 {
                        break;
                    }

                    snippet.add_lang(section);

                    if j == 0 {
                        let mut name = snippet
                            .dirs
                            .last()
                            .unwrap()
                            .content
                            .split(" ")
                            .collect::<Vec<&str>>()[0]
                            .trim()
                            .replace('å', "aa")
                            .replace("ø", "oe")
                            .replace("æ", "ae")
                            .to_string();

                        name.retain(|x| !['*', '{', '}'].contains(&x));
                        name.retain(|x| x.is_alphanumeric());

                        self.names.push(name)
                    }
                }

                self.content.push(snippet)
            }
        }
    }

    pub fn as_i18_yaml(&self) -> Vec<String> {
        let mut actual_result = Vec::new();

        for j in 0 .. self.content.len() {
            let mut result = format!("{}:\n", self.path);

            let snippet = &self.content[j];

            for (i, lang) in snippet.dirs.iter().enumerate() {

                if lang.content.contains('{') {
                    continue;
                }
                
                result.push_str(&format!(
                    "\t__{}_{}__{}: \"{}\"",
                    self.names[j].trim(),
                    i,
                    lang.tag,
                    lang.to_html().trim()
                ));

                result.push('\n')
            }

            actual_result.push(result)
        }

        actual_result
    }
}
