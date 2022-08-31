use gtk::gdk;
use gtk::glib;

mod imp {
    use gtk::gdk::Snapshot;
    use gtk::glib::ffi::G_PI;
    use gtk::glib::Cast;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, PaintableImpl};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{gdk, glib, graphene};
    use std::cell::Cell;

    #[derive(Default)]
    pub struct SimpleDial {
        pub width: Cell<i32>,
        pub height: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleDial {
        const NAME: &'static str = "SimpleDial";
        type Type = super::SimpleDial;
        type ParentType = gtk::Widget;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for SimpleDial {}
    impl PaintableImpl for SimpleDial {
        fn snapshot(&self, _paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64) {
            println!("=============== snapshot ===============");
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&rect);

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

            cr.stroke();
        }
    }
    impl WidgetImpl for SimpleDial {}

    fn point_on_arc(r: f64, i: i32) -> (f64, f64) {
        (
            r * (i as f64 * G_PI / 6.0).cos(),
            r * (i as f64 * G_PI / 6.0).sin(),
        )
    }
}

glib::wrapper! {
    pub struct SimpleDial(ObjectSubclass<imp::SimpleDial>)
        @extends gtk::Widget,
        @implements gdk::Paintable;
}

impl Default for SimpleDial {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleDial {
    pub fn new() -> Self {
        let paintable: Self = glib::Object::new(&[]).unwrap();
        paintable
    }
}
