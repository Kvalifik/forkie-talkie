#![feature(async_await)]

#[macro_use]
extern crate serde_json;

use std::env;
use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::path::Path;

mod forkie;

use forkie::page::*;
use forkie::forkie::internationalize;

extern crate crossbeam;

// TODO: don't hardcode language codes
pub fn grab_language_md(path: &str, name: &str, md_path: &str) {
    let path_a = format!("{}/{}_da.md", md_path, name);
    let path_b = format!("{}/{}_en.md", md_path, name);

    let content_a = file_content(&path_a);
    let content_b = file_content(&path_b);

    let mut page = Page::new();
    let group = vec![("da".to_string(), content_a), ("en".to_string(), content_b)];

    page.add_files(vec!(group));

    let mut html = file_content(path);

    internationalize(&mut html, &page);
    html.push_str(&format!("\n\n{}", page.as_i18_yaml()));

    let mut output_file = File::create(&path).unwrap();
    match output_file.write_all(html.as_bytes()) {
        Ok(_)    => (),
        Err(why) => println!("{}", why)
    }
}

pub fn grab_path(path: &str, md_path: &str) {
    let meta = match metadata(path) {
        Ok(data) => data,
        Err(why) => panic!("{}", why),
    };

    if meta.is_file() {
        let split: Vec<&str> = path.split('.').collect();

        if *split.last().unwrap() == "vue" {
            grab_language_md(path, split[0].split('/').last().unwrap(), md_path);
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        crossbeam::scope(|spawner| {
            for folder_path in paths {
                spawner.spawn(|_| {
                    let folder_path = format!("{}", folder_path.unwrap().path().display());
                    let split: Vec<&str> = folder_path.split('.').collect();

                    if Path::new(&folder_path).is_dir() || *split.last().unwrap() == "vue" {
                        grab_path(&folder_path, md_path)
                    }
                });
            }
        }).unwrap();
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

const HELP: &'static str = r#"
Forkie forkie talkie

USAGE:
    forkie <file/folder> <folder>
"#;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        grab_path(&args[1], &args[2])
    } else {
        println!("{}", HELP)
    }
}