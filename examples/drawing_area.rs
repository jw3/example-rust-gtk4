use std::str::FromStr;

use crate::gdk::pango;
use crate::gdk::pango::FontDescription;
use gtk::gdk;
use gtk::gdk::RGBA;
use gtk::glib::ffi::G_PI;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.flowbox"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("FlowBox")
        .build();

    let drawing_area = gtk::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .build();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
        cr.paint().unwrap();

        let (xc, yc) = (width as f64 / 2.0, height as f64 / 2.0);
        println!("{}, {}", xc, yc);

        cr.set_line_width(1.0);

        cr.set_source_rgb(1.0, 1.0, 1.0);

        cr.rectangle(xc, yc, 1.0, 1.0);
        cr.stroke().unwrap();

        let radius = 50.0;
        cr.arc_negative(xc, yc, radius, 1.0, 2.0);
        cr.stroke().unwrap();

        cr.save();
        cr.translate(xc, yc);
        cr.rotate(2.25);

        let inset = 10.0;

        cr.set_font_size(18.0);
        for i in 0..10 {
            let inner_tick = point_on_arc(radius - inset, i);
            cr.move_to(inner_tick.0, inner_tick.1);
            let outer_tick = point_on_arc(radius, i);
            cr.line_to(outer_tick.0, outer_tick.1);
            cr.stroke();

            // cr.save();
            // let text_pos = point_on_arc(radius + 10.0, i);
            // cr.move_to(text_pos.0 - (i as f64 * 2.0) + 15.0, text_pos.1 + 20.0);
            // cr.rotate(4.0);
            // cr.show_text(format!("{}", i * 10).as_str());
            // cr.stroke();
            // cr.restore();
        }
        cr.restore();

        cr.move_to(xc, yc);
        cr.line_to(xc + 20.0, yc + 20.0);
        cr.stroke();
    });

    window.set_child(Some(&drawing_area));
    window.show();
}

fn point_on_arc(r: f64, i: i32) -> (f64, f64) {
    (
        r * (i as f64 * G_PI / 6.0).cos(),
        r * (i as f64 * G_PI / 6.0).sin(),
    )
}
