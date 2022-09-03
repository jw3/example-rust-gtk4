use gtk::prelude::RangeExt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;
    let range = gtk::Scale::default();

    range.set_range(0.0, 100.0);
    range.set_value(25.0);
    range.set_fill_level(99.0);
    println!("fill level: {}", range.fill_level());

    Ok(())
}
