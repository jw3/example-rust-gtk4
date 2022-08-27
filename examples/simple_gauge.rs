use std::thread::{sleep, Thread};
use std::time::Duration;
use gtk::{Builder, ApplicationWindow};
use gtk::prelude::*;

use example_gtk4::simple_gauge::SimpleGauge;

fn main() {
    SimpleGauge::ensure_type();

    let application = gtk::Application::new(
        None,
        Default::default(),
    );

    application.connect_activate(|app| {
        let glade_src = include_str!("../src/gauge.glade");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.object("main_window").unwrap();
        window.set_application(Some(app));
        window.present();
    });

    application.run();
}
