use scraper::Html;
use std::{env, fs::{File, self}, io};

mod ehentai;

pub struct Manga {
    title: String,
    pages: Vec<String>
}

impl Manga {
    async fn manga_download(self) {
        fs::create_dir_all(&self.title).unwrap();
        for img in self.pages.iter() {
            let filename = self.title.to_string() + "/" + img.split("/").last().unwrap();
            let response = reqwest::get(img).await.unwrap();
            let bytes = response.bytes().await.unwrap();
            let mut out = File::create(filename).unwrap();
            io::copy(&mut bytes.as_ref(), &mut out).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let arg1: String = get_url_to_args();
    let result: String = get_reqwest(&arg1).await.unwrap();
    let ehantai_html: Html = Html::parse_document(&result);

    let manga = ehentai::get_ehentai(&ehantai_html).await;
    
    manga.manga_download().await;
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


