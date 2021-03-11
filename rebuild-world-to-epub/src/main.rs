use scraper::{Html, Selector};

struct Chapter {
    name: String,
    link: String,
}
type Index = Vec<Chapter>;
struct Page {
    title: String,
    content: Vec<String>,
}

fn main() {
    let pages = get_index();
    /*for page in pages.iter() {
        println!("{} - {}", page.name, page.link);
    }*/
    let page = &pages[0];
    println!("{:?}", get_page(&page.link));
}

fn get_page(link: &str) -> Vec<String> {
    let page = get_url(link);
    let document = Html::parse_document(&page);

    let title_selector = Selector::parse("h1.entry-title").unwrap();
    let text_selector = Selector::parse("div.entry-content").unwrap();
    let p_selector = Selector::parse("p").unwrap();

    let content = document.select(&text_selector).next().unwrap();
    let mut result: Vec<String> = Vec::new();

    let stop_wrods = [
        "\u{a0}",
        "Index ",
        "Translator: ",
        "Editor",
        "Proofreader: ",
    ];

    for item in content.select(&p_selector) {
        let mut result_line = String::new();
        for line in item.text() {
            if stop_wrods.contains(&line) {
                break;
            }
            result_line.push_str(line);
        }

        if !result_line.is_empty() {
            result.push(result_line)
        }
    }

    result
}

fn get_index() -> Index {
    let mut result: Index = Vec::new();

    let index_page = get_url("https://www.divinedaolibrary.com/rebuild-world/");
    let document = Html::parse_document(&index_page);

    let chpaters_selector =
        Selector::parse(r#"a[style="color: #ff9900; --darkreader-inline-color: #ffa31a;"]"#)
            .unwrap();

    for element in document.select(&chpaters_selector) {
        let name = element.inner_html();
        let link = String::from(element.value().attr("href").unwrap());
        result.push(Chapter { name, link });
    }

    result
}

fn get_url(url: &str) -> String {
    fn try_get_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(reqwest::blocking::get(url)?.text()?)
    }
    let mut last_err: Box<dyn std::error::Error> = Box::new(std::fmt::Error::default());
    for _ in 0..3 {
        match try_get_page(url) {
            Ok(text) => return text,
            Err(err) => last_err = err,
        }
    }
    panic!("Failed to get page {}\nError: {}", url, last_err);
}
