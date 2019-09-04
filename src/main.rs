#[macro_use]
extern crate serde_json;

mod forkie;

use forkie::page::*;
use forkie::forkie::internationalize;

fn main() {
    let md_da = r#"
### jeg hedder walkie talkie
---
[en knap]
---
[bobs]
    "#;

    let md_en = r#"
### my name is forkie forkie talkie
---
[a button]
---

[weird]
    "#;

    let mut html = r#"
<h3>jeg hedder kkkaj</h3>
<v-btn>butt 1</v-btn>
<v-btn>button 2</v-btn>

    "#.to_string();

    let mut page = Page::new();
    let group = vec![("da".to_string(), md_da.to_owned()), ("en".to_string(), md_en.to_owned())];

    page.add_files(vec!(group));

    println!("{}", page.as_i18_yaml());

    internationalize(&mut html, &page);

    println!("{}", html)
}
