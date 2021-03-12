mod parser;

fn main() {
    let parser = parser::BookParser::new();
    let pages = parser.get_index();
    /*for page in pages.iter() {
        println!("{} - {}", page.name, page.link);
    }*/
    let page = &pages[0];
    let page = parser.get_page(&page.link);
    println!("{:?}", page);
}