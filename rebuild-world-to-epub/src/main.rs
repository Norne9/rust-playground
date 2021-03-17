mod parser;
use indicatif::{ProgressBar, ProgressStyle};
use parser::Page;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use std::{convert::TryInto, usize};

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
        pb.set_message(&format!("Downloading `{}`", page.name));
        pb.inc(1);
    }

    //pb.set_message("Processing");
    match write_book(&chapters) {
        Ok(_) => pb.finish_with_message("Done!"),
        Err(err) => {
            pb.finish_with_message("Error!");
            println!("Failed to save file: {}", err)
        }
    };
}

fn write_book(chapters: &Vec<parser::Page>) -> Result<(), Error> {
    // Create "Chapters" directory
    if !Path::new("./Chapters").exists() {
        std::fs::create_dir("./Chapters")?;
    }

    // open file
    let zip_file = File::create("rebuild-world.fb2.zip")?;
    let mut zip = zip::ZipWriter::new(zip_file);

    //let options =
    //    zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
    zip.start_file("rebuild-world.fb2", Default::default())?;

    // write file header
    let first_part = include_str!("start.fb2");
    write!(zip, "{}", first_part)?;

    // write chapters
    write!(zip, "<body>")?;
    for (i, chapter) in chapters.iter().enumerate() {
        // Write to fb2
        write!(zip, "<section>")?;
        write!(
            zip,
            "<title><p>{}</p></title>",
            filter_symbols(&chapter.title)
        )?;
        for line in chapter.content.iter() {
            match line {
                parser::Content::Line(text) => write!(zip, "<p>{}</p>", filter_symbols(text))?,
                parser::Content::Break => write!(zip, "<empty-line></empty-line>")?,
            }
        }
        write!(zip, "</section>")?;

        // Write to txt
        write_text_file(i, chapter)?;
    }
    write!(zip, "</body>")?;
    write!(zip, "</FictionBook>")?;

    // send all pending changes
    zip.finish()?;
    Ok(())
}

fn filter_symbols(text: &str) -> String {
    String::from(text)
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("&", "&amp;")
}

fn write_text_file(index: usize, page: &Page) -> Result<(), Error> {
    let name = format!(
        "./Chapters/{:05} - {}.txt",
        index,
        filter_filename(&page.title)
    );

    let file = File::create(name)?;
    let mut file = BufWriter::new(file);

    let title = page.title.replace("–", "\n").replace(",", "\n");
    writeln!(file, "{}", title)?;

    for line in page.content.iter() {
        match line {
            parser::Content::Line(text) => write!(file, "{}", text)?,
            parser::Content::Break => write!(file, "\n")?,
        }
    }

    file.flush()?;
    Ok(())
}

fn filter_filename(name: &str) -> String {
    name.replace("<", "_")
        .replace(">", "_")
        .replace(":", "_")
        .replace("\"", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace("|", "_")
        .replace("?", "_")
        .replace("*", "_")
}
