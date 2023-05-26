use log::info;
use std::{env, fs::{File, self}};
use std::io::{BufWriter, copy};
use std::collections::HashMap;

mod ehentai;

pub struct Manga {
    title: String,
    pages: HashMap<u16, String>
}

impl Manga {
    async fn manga_download(self) {
        fs::create_dir_all(&self.title).unwrap();
        for (pagenum, img) in self.pages.iter() {
            info!("{}", &img);
            let filename = self.title.to_string() + "/" + &pagenum.to_string() + "_" + img.split("/").last().unwrap();
            let response = reqwest::get(img).await.unwrap();
            let bytes = response.bytes().await.unwrap();
            let mut out: BufWriter<File> = BufWriter::new(File::create(filename).unwrap());
            copy(&mut bytes.as_ref(), &mut out).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let arg1: String = get_url_to_args();
    let manga = ehentai::get_ehentai(&arg1).await;
    
    manga.manga_download().await;
}

fn get_url_to_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let d: &String = args.get(1).unwrap();
    return d.to_string();
}
