use std::collections::{HashSet, HashMap};
use scraper::{Html, Selector};
use log::{info};
use regex::Regex;

use super::Manga;

async fn get_reqwest(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

pub async fn get_ehentai(url: &str) -> Manga {
    let result: String = get_reqwest(&url).await.unwrap();
    let html: Html = Html::parse_document(&result);

    let manga_name: String = get_manga_name(&html);
    info!("【Name】{}", &manga_name);

    let external_viewer_links: Option<Vec<String>> = get_external_viewer_links(&html);
    let mut img_links: HashMap<u16, String> = HashMap::new();

    for img in get_all_imglink(&html) {
        info!("【img】{}:{}", 1, &img);
        let (pagenum, imglink) = single_page_scraper(&img).await;
        img_links.insert(pagenum, imglink);
    }

    if let Some(viewer_page_links) = external_viewer_links {
        for viewer in viewer_page_links {
            info!("【viewer】:{}", &viewer);
            let response: String = get_reqwest(&viewer).await.unwrap();
            let current_html: Html = Html::parse_document(&response);
            let all_imglink = get_all_imglink(&current_html);
            for img in all_imglink {
                info!("【img】{}:{}", "external", &img);
                let (pagenum, imglink) = single_page_scraper(&img).await;
                img_links.insert(pagenum, imglink);
            }
        }
    }

    Manga { title: manga_name, pages: img_links }
}

fn get_manga_name(html: &Html) -> String {
    let selector_str: &str = "#gj, #gn";
    let selector: Selector = Selector::parse(selector_str).unwrap();

    for element in html.select(&selector) {
        let title = element.inner_html();
        if !title.is_empty() {
            return title;
        }
    }

    "Untitled".to_string()
}

fn get_all_imglink(html: &Html) -> HashSet<String> {
    let gdt_selector: &str = "#gdt div a";
    let selector: Selector = Selector::parse(gdt_selector).unwrap();
    let mut imglinks: HashSet<String> = HashSet::new();

    for element in html.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            imglinks.insert(link.to_string());
        }
    }

    imglinks
}

fn get_external_viewer_links(html: &Html) -> Option<Vec<String>> {
    let selector_td: Selector = Selector::parse("body > div:nth-child(10) > table > tbody > tr td a").unwrap();
    let element_tds = &mut html.select(&selector_td);
    let last_page_a = element_tds.rev().nth(1)?.value().attr("href")?;
    let re = Regex::new(r"https://e-hentai.org/g/([a-zA-Z0-9]+)/([a-zA-Z0-9]+)/\?p=(\d+)").unwrap();

    if let Some(captures) = re.captures(&last_page_a) {
        let content_link = format!("https://e-hentai.org/g/{}/{}", &captures[1], &captures[2]);
        let last_page_num = captures[3].parse::<u8>().ok()?;
        let viewer_links: Vec<String> = (1..=last_page_num).map(|i| format!("{}?p={}", &content_link, i)).collect();
        Some(viewer_links)
    } else {
        None
    }
}

async fn single_page_scraper(url: &str) -> (u16, String) {
    let pagenum = url_extract_pagenum(&url);
    let response = get_reqwest(&url).await.unwrap();
    let html: Html = Html::parse_document(&response);
    let selector = Selector::parse("#img").unwrap();
    let img_src = html.select(&selector).next().unwrap().value().attr("src").unwrap();
    (pagenum, img_src.to_string())
}

fn url_extract_pagenum(url: &str) -> u16 {
    let pagenum_matchpattern = regex::Regex::new(r"/\d+-(\d+)").unwrap();
    if let Some(i) = pagenum_matchpattern.captures(&url) {
        return i[1].parse().unwrap_or(0);
    } 

    0
}