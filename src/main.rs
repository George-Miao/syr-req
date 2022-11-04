use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("data/data.html")?;
    let res = scraper::Html::parse_document(&data);
    res.select(&s!("123")).for_each(|x| println!("{x:#?}"));

    Ok(())
}

macro_rules! s {
    ($in:literal) => {{ ::scraper::Selector::parse($in).expect("invalid selector") }};
}

use s;
