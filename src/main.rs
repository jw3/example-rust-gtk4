mod simple_gauge;

use std::thread::{sleep, Thread};
use std::time::Duration;
use gtk::{Builder, Window};
use gtk::prelude::*;
use crate::simple_gauge::SimpleGauge;

fn main() {
    SimpleGauge::ensure_type();

    let application = gtk::Application::new(
        None,
        Default::default(),
    );

    application.connect_activate(|app| {
        let glade_src = include_str!("gauge.glade");
        let builder = Builder::from_string(glade_src);
        let window: Window = builder.object("main_window").unwrap();
        window.set_application(Some(app));
        window.present();
    });

    application.run();
}
