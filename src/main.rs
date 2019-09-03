mod forkie;

use forkie::page::*;

fn main() {
    let md_da = r#"
### jeg hedder bob bob talkie
---
[en knap]
---
# skrrrrtttt
---
## NETTO nuts
---
[  knaaap igen ]
    "#;

    let md_en = r#"
### my name is bob
---
[a but]
---
# car sound
---
## netto-nødder
---
[but igen]
    "#;

    let mut page = Page::new();
    let group = vec![("da".to_string(), md_da.to_owned()), ("en".to_string(), md_en.to_owned())];

    page.add_files(vec!(group));

    println!("{:#?}", page.names);

    println!("{:#?}", page.content)
}
