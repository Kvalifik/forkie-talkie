#![feature(async_await)]

#[macro_use]
extern crate serde_json;
extern crate onig;

use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;

mod page;
use page::*;

use onig::*;

extern crate crossbeam;

pub fn grab_language_md(path: &str, name: &str, md_path: &str, yaml_path: &str) {
    let path_a = format!("{}/{}_da.md", md_path, name);
    let path_b = format!("{}/{}_en.md", md_path, name);

    let content_a = file_content(&path_a);
    let content_b = file_content(&path_b);

    if content_a.is_none() || content_b.is_none() {
        return;
    }

    let mut page = Page::new(name.to_owned());
    let group = vec![
        ("da".to_string(), content_a.unwrap()),
        ("en".to_string(), content_b.unwrap()),
    ];

    page.add_files(vec![group]);

    let mut html = file_content(path).unwrap();

    for (i, content) in page.content[0].dirs.iter().enumerate() {
        if content.content.contains('{') {
            continue;
        }

        let reg = match Regex::new(&format!(
            "(?=[_\"\\w>])({}*)(?![_\"\\w>])",
            content.to_trim().trim()
        )) {
            Ok(r) => r,
            Err(e) => return println!("{}", e),
        };

        html = reg.replace(
            &html,
            format!(
                " {{{{  $t('__{}_{}__{}')  }}}}",
                name, i, content.tag
            )
            .as_str(),
        );
    }

    // output two yaml files, easily scalable to infinitely, exponential, AI, conjoined triangles of success and kubernetes many
    // getting a bit hard-cody up in here
    // TODO: ADD MORE ARGS
    
    let mut file_da = File::create(format!("{}/da.dojo.yaml", yaml_path)).unwrap();
    let mut file_en = File::create(format!("{}/en.dojo.yaml", yaml_path)).unwrap();

    let page_yaml = page.as_i18_yaml();

    file_da.write_all(page_yaml[0].as_bytes()).expect("Fucked up danish YAML");
    file_en.write_all(page_yaml[1].as_bytes()).expect("Fucked up english YAML");

    // end of kinda bad YAML area;

    let mut output_file = File::create(&path).unwrap();

    match output_file.write_all(html.as_bytes()) {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
}

pub fn grab_path(path: &str, md_path: &str, yaml_path: &str) {
    let meta = match metadata(path) {
        Ok(data) => data,
        Err(why) => panic!("{}", why),
    };

    if meta.is_file() {
        let split: Vec<&str> = path.split('.').collect();

        if *split.last().unwrap() == "vue" {
            grab_language_md(path, split[0].split('/').last().unwrap(), md_path, yaml_path);
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        crossbeam::scope(|spawner| {
            for folder_path in paths {
                spawner.spawn(|_| {
                    let folder_path = format!("{}", folder_path.unwrap().path().display());
                    let split: Vec<&str> = folder_path.split('.').collect();

                    if Path::new(&folder_path).is_dir() || *split.last().unwrap() == "vue" {
                        grab_path(&folder_path, md_path, yaml_path)
                    }
                });
            }
        })
        .unwrap();
    }
}

pub fn file_content(path: &str) -> Option<String> {
    let display = Path::new(path).display();

    let mut file = match File::open(&path) {
        Err(why) => return None,
        Ok(file) => Some(file),
    };

    let mut s = String::new();

    match file.unwrap().read_to_string(&mut s) {
        Err(why) => None,
        Ok(_) => Some(s.to_owned()),
    }
}

const HELP: &'static str = r#"
Forkie forkie talkie

USAGE:
    forkie <file/folder> <folder> <path to yaml folder>
"#;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 3 {
        grab_path(&args[1], &args[2], &args[3])
    } else {
        println!("{}", HELP)
    }
}
