use handlebars::Handlebars;
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use comrak::{
    markdown_to_html,
    ComrakOptions
};

fn main() {
    println!("Hello, world!");
    let name = "neutralboy";
    let mut handlebars = Handlebars::new();


    handlebars.register_template_file("template_name1", Path::new("./templates/article.html")).unwrap();

    
    let md_read = fs::read_to_string(Path::new("./Readme.md")).unwrap();
    let md_str: &str = &md_read;
    let converted_md: &str = &markdown_to_html(md_str, &ComrakOptions::default());


    let mut data = HashMap::new();
    data.insert("name", name);
    data.insert("mdContents", converted_md);

    let mut file = File::create(Path::new("./page/article.html")).unwrap();


    let rendered = handlebars.render("template_name1", &data).unwrap();
    file.write_all(rendered.as_bytes()).unwrap();


    println!("{}", rendered);
}
