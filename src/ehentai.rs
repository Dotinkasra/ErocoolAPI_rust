use std::collections::HashSet;
use log::{info, warn, error, trace};

use scraper::Html;
use scraper::Selector;
use regex::Regex;

use super::Manga;

pub async fn get_ehentai(html: &Html) -> Manga {
    let manga_name: String = get_manga_name(html);
    info!("【Name】:{}", &manga_name);

    let viewer_page_links: Vec<String> = get_viewer_links(html);
    let img_links: &mut Vec<String> = &mut vec![];
    
    for viewer in viewer_page_links {
        info!("【viewer】:{}", &viewer);
        let response = reqwest::get(viewer).await.unwrap().text().await.unwrap();
        let html: Html = Html::parse_document(&response);
        let all_imglink = get_all_imglink(&html);
        for img in all_imglink {
            info!("【img】:{}", &img);
            img_links.push(single_page_scraper(&img).await);
        }
    }
    return Manga{title: manga_name, pages: img_links.to_vec()};
}

fn get_manga_name(html: &Html) -> String {
    let selector_str: &str = "#gj";
    let selector: Selector = Selector::parse(selector_str).unwrap();

    for element in html.select(&selector) {
        return element.inner_html();
    }
    return "Untitled".to_string();
}

fn get_all_imglink(html: &Html) -> HashSet<String> {
    let gdt_selector: &str = "#gdt div";
    let selector: Selector = Selector::parse(gdt_selector).unwrap();
    let mut imglinks: Vec<String> = vec![];

    for element in html.select(&selector) {
        let alink_selector: Selector = Selector::parse("a").unwrap();
        for alink in element.select(&alink_selector) {
            let link = alink.value().attr("href").unwrap();
            imglinks.push(link.to_string());
        }
    }
    let uniq_imglinks: HashSet<String> = imglinks.into_iter().collect();
    return uniq_imglinks;
}

fn get_viewer_links(html: &Html)  -> Vec<String>{
    let mut viewer_links: Vec<String> = vec![];

    let selector_td: Selector = Selector::parse("body > div:nth-child(10) > table > tbody > tr td").unwrap();
    let selector_a: Selector = Selector::parse("a").unwrap();
    
    let element_tds: &mut scraper::html::Select = &mut html.select(&selector_td);
    let element_lastpage: scraper::ElementRef = element_tds.rev().nth(1).unwrap();

    let last_page_a: &mut scraper::element_ref::Select = &mut element_lastpage.select(&selector_a);
    let i = last_page_a.next().unwrap();
    let url = i.value().attr("href").unwrap();

    let re = Regex::new(r"https://e-hentai.org/g/([a-zA-Z0-9]+)/([a-zA-Z0-9]+)/\?p=(\d+)").unwrap();

    // Some(変数名) の形であれば
    if let Some(captures) = re.captures(&url) {
        let content_link = String::from("https://e-hentai.org/g/") + &captures[1] + "/" + &captures[2] + "/";
        let template = content_link.clone();

        viewer_links.push(content_link);

        let last_page_num = &captures[3].parse::<u8>().unwrap();

        for i in 1..=*last_page_num {
            let numbered_url = template.to_string() + "p=" + &i.to_string();
            viewer_links.push(numbered_url);
        }
    } else {
        viewer_links.push(url.to_string());
    }
    return viewer_links;

}

async fn single_page_scraper(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();
    let html: Html = Html::parse_document(&response);
    let selector = Selector::parse("#img").unwrap();
    let img_select = &mut html.select(&selector);
    let img_src = img_select.nth(0)
                            .unwrap()
                            .value()
                            .attr("src")
                            .unwrap();
    return img_src.to_string();
}