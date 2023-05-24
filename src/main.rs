use scraper::Html;
use std::env;

mod ehentai;
struct Manga {
    url: String,
    title: String,
    pages: Vec<String>,
    total_pages_num: i128
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let arg1: String = get_url_to_args();
    let result: String = get_reqwest(&arg1).await?;
    let ehantai_html: Html = Html::parse_document(&result);

    let module_test = ehentai::get_manga_name(&ehantai_html);
    println!("{module_test}");

    Ok(())
}

async fn get_reqwest(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

fn get_url_to_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let d: &String = args.get(1).unwrap();
    return d.to_string();
}


