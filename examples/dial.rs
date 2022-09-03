use gauges::layered_gauge::LayeredGauge;
use gauges::parts::simple_dial::SimpleDial;
use gtk::builders::PictureBuilder;
use gtk::prelude::*;
use gtk::Picture;

fn main() {
    let application = gtk::Application::new(None, Default::default());
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .default_width(600)
            .default_height(600)
            .application(app)
            .build();

        let dial = SimpleDial::default();
        let pic = PictureBuilder::new().paintable(&dial).build();
        window.set_child(Some(&pic));
        window.show();
    });
    application.run();
}
