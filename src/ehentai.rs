use scraper::Html;
use scraper::Selector;

pub fn get_manga_name(html: &Html) -> String {
    let imglinks = get_all_imglink(html);
    for imglink in imglinks {
        println!("{}", imglink);
    }
    let selector_str: &str = "#gj";
    let selector: Selector = Selector::parse(selector_str).unwrap();
    for element in html.select(&selector) {
        return element.inner_html();
    }
    return "Untitled".to_string();
}


pub fn get_all_imglink(html: &Html) -> Vec<String> {
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