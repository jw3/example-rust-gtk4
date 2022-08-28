use gauges::simple_gauge::SimpleGauge;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

fn main() {
    SimpleGauge::ensure_type();

    let application = gtk::Application::new(None, Default::default());

    application.connect_activate(|app| {
        let glade_src = include_str!("../src/gauge.glade");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.object("main_window").unwrap();
        window.set_application(Some(app));
        window.present();
    });

    application.run();
}
