use scraper::{Html, Selector};

pub struct BookParser {
    title_selector: Selector,
    text_selector: Selector,
    p_selector: Selector,
    chpaters_selector: Selector,
    client: reqwest::blocking::Client,
    stop_words: Vec<&'static str>,
}
pub struct Chapter {
    pub name: String,
    pub link: String,
}
pub type Index = Vec<Chapter>;
#[derive(Debug)]
pub struct Page {
    pub title: String,
    pub content: Vec<Content>,
}
#[derive(Debug)]
pub enum Content {
    Line(String),
    Break,
}

impl BookParser {
    pub fn new() -> BookParser {
        BookParser {
            title_selector: Selector::parse("h1.entry-title").unwrap(),
            text_selector: Selector::parse("div.entry-content").unwrap(),
            p_selector: Selector::parse("p").unwrap(),
            chpaters_selector: Selector::parse(
                r#"a[style="color: #ff9900; --darkreader-inline-color: #ffa31a;"]"#,
            )
            .unwrap(),
            client: reqwest::blocking::Client::new(),
            stop_words: vec![
                //"\u{a0}",
                "Index ",
                "Translator: ",
                "Editor",
                "Proofreader: ",
            ],
        }
    }

    // https://www.divinedaolibrary.com/rebuild-world-chapter-1-akira-and-alpha/
    pub fn get_page(&self, link: &str) -> Page {
        let page = self.get_url(link);
        let document = Html::parse_document(&page);

        let title = document.select(&self.title_selector).next().unwrap();
        let content = document.select(&self.text_selector).next().unwrap();

        let mut lines = Vec::<Content>::new();
        for item in content.select(&self.p_selector) {
            let mut result_line = String::new();
            for line in item.text() {
                if self.stop_words.contains(&line) {
                    break;
                } else if line == "\u{a0}" {
                    lines.push(Content::Break);
                    break;
                }
                result_line.push_str(line);
            }

            if !result_line.is_empty() {
                lines.push(Content::Line(result_line));
            }
        }

        Page {
            title: title.inner_html(),
            content: lines,
        }
    }

    // https://www.divinedaolibrary.com/rebuild-world/
    pub fn get_index(&self) -> Index {
        let mut result: Index = Vec::new();

        let index_page = self.get_url("https://www.divinedaolibrary.com/rebuild-world/");
        let document = Html::parse_document(&index_page);

        for element in document.select(&self.chpaters_selector) {
            let name = element.inner_html();
            let link = String::from(element.value().attr("href").unwrap());
            result.push(Chapter { name, link });
        }

        result
    }

    fn get_url(&self, url: &str) -> String {
        fn try_get_page(
            client: &reqwest::blocking::Client,
            url: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            Ok(client.get(url).send()?.text()?)
        }
        let mut last_err: Box<dyn std::error::Error> = Box::new(std::fmt::Error::default());
        for _ in 0..3 {
            match try_get_page(&self.client, url) {
                Ok(text) => return text,
                Err(err) => last_err = err,
            }
        }
        panic!("Failed to get page {}\nError: {}", url, last_err);
    }
}
