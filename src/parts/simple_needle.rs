use gtk::gdk;
use gtk::glib;

mod imp {
    use gtk::gdk::Snapshot;
    use gtk::glib::Cast;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, PaintableImpl};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{gdk, glib, graphene};
    use std::cell::Cell;

    #[derive(Default)]
    pub struct SimpleNeedle {
        pub width: Cell<i32>,
        pub height: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleNeedle {
        const NAME: &'static str = "SimpleNeedle";
        type Type = super::SimpleNeedle;
        type ParentType = gtk::Widget;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for SimpleNeedle {}
    impl PaintableImpl for SimpleNeedle {
        fn snapshot(&self, _paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64) {
            println!("=============== snapshot ===============");
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&rect);

            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.paint().unwrap();

            cr.set_source_rgba(0.0, 1.0, 0.0, 1.0);

            let (xc, yc) = (width as f64 / 2.0, height as f64 / 2.0);
            println!("{}, {}", xc, yc);
            cr.move_to(xc, yc);
            cr.line_to(xc + 20.0, yc + 20.0);
            cr.stroke();
        }
    }
    impl WidgetImpl for SimpleNeedle {}
}

glib::wrapper! {
    pub struct SimpleNeedle(ObjectSubclass<imp::SimpleNeedle>)
        @extends gtk::Widget,
        @implements gdk::Paintable;
}

impl Default for SimpleNeedle {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleNeedle {
    pub fn new() -> Self {
        let paintable: Self = glib::Object::new(&[]).unwrap();
        paintable
    }
}
