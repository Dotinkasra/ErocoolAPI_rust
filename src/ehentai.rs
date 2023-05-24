use scraper::Html;
use scraper::Selector;
use regex::Regex;

pub fn get_manga_name(html: &Html) -> String {
    let selector_str: &str = "#gj";
    let selector: Selector = Selector::parse(selector_str).unwrap();
    for element in html.select(&selector) {
        return element.inner_html();
    }
    return "Untitled".to_string();
}

fn get_all_imglink(html: &Html) -> Vec<String> {
    let gdt_selector: &str = "#gdt div";
    let selector: Selector = Selector::parse(gdt_selector).unwrap();
    let mut imglinks: Vec<String> = vec![];

    for element in html.select(&selector) {
        println!("{}", &element.inner_html());
        let alink_selector: Selector = Selector::parse("a").unwrap();
        for alink in element.select(&alink_selector) {
            let link = alink.value().attr("href").unwrap();
            imglinks.push(link.to_string());
        }
    }
    return imglinks;
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
            println!("{}", i);
            let url = template.to_string() + "p=" + &i.to_string();
            viewer_links.push(url);
        }
    }
    return viewer_links;

}

