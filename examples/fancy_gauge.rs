use gauges::rendered_gauge::RenderedGauge;
use gtk::prelude::*;

fn main() {
    RenderedGauge::ensure_type();

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.paintable"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(350, 350);

    let paintable = RenderedGauge::new();

    let picture = gtk::Picture::new();
    picture.set_halign(gtk::Align::Center);
    picture.set_size_request(200, 200);
    picture.set_paintable(Some(&paintable));

    window.set_child(Some(&picture));
    window.show();
}
