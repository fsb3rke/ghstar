use reqwest::Error;
use scraper::{Html, Selector};
use std::env::args;

fn get(arg: Vec<String>, i: usize) -> String {
    let val: Option<&String> = arg.get(i);
    match val {
        Some(n) => n.to_string(),
        None => String::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let collected_args: Vec<String> = args().collect();
    let resp = reqwest::get(get(collected_args, 1)).await?;
    println!("Status: {}", resp.status());
    let body = resp.text().await?;
    let doc = Html::parse_document(&body);
    let sel = Selector::parse("strong").expect("HUH");

    doc.select(&sel).enumerate().for_each(|(i, e)| {
        if i == 1 {
            println!("{}", e.inner_html());
        }
    });

    Ok(())
}
