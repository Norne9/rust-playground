mod parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

fn main() {
    let parser = parser::BookParser::new();
    let pages = parser.get_index();

    let pb = ProgressBar::new(pages.len().try_into().unwrap());
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>4}/{len:4} ({eta})")
        .progress_chars("#>-"));

    let mut chapters = Vec::<parser::Page>::new();
    for page in pages.iter() {
        chapters.push(parser.get_page(&page.link));
        pb.set_message(format!("Downloading `{}`", page.name));
        pb.inc(1);
    }

    pb.set_message("Processing");
    match write_book(&chapters) {
        Ok(_) => pb.finish_with_message("Done!"),
        Err(err) => {
            pb.finish_with_message("Error!");
            println!("Failed to save file: {}", err)
        }
    };
}

fn write_book(chapters: &Vec<parser::Page>) -> Result<(), Error> {
    // open file
    let file = File::create("rebuild-world.fb2")?;
    let mut file = BufWriter::new(file);

    // write file header
    let first_part = include_str!("start.fb2");
    write!(file, "{}", first_part)?;

    // write chapters
    write!(file, "<body>")?;
    for chapter in chapters.iter() {
        write!(file, "<section>")?;
        write!(file, "<title><p>{}</p></title>", chapter.title)?;
        for line in chapter.content.iter() {
            match line {
                parser::Content::Line(text) => write!(file, "<p>{}</p>", text)?,
                parser::Content::Break => write!(file, "<empty-line />")?,
            }
        }
        write!(file, "</section>")?;
    }
    write!(file, "</body>")?;
    write!(file, "</FictionBook>")?;

    // send all pending changes
    file.flush()?;
    Ok(())
}
