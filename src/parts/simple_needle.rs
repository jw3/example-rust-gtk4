use gtk::gdk;
use gtk::glib;
use gtk::glib::ObjectExt;
use gtk::prelude::{PaintableExt, WidgetExt};
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclassIsExt};

mod imp {
    use crate::{double_prop, integer_prop, point_to_arc};
    use gtk::gdk::Snapshot;
    use gtk::glib::{Cast, ParamSpec, ParamSpecDouble, ParamSpecInt, ParamSpecIntBuilder, Value};
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, PaintableImpl};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{gdk, glib, graphene};
    use once_cell::sync::Lazy;
    use std::cell::Cell;

    #[derive(Default)]
    pub struct SimpleNeedle {
        pub radius: Cell<f64>,
        pub min: Cell<f64>,
        pub max: Cell<f64>,
        pub start: Cell<i32>,
        pub end: Cell<i32>,
        pub val: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleNeedle {
        const NAME: &'static str = "SimpleNeedle";
        type Type = super::SimpleNeedle;
        type ParentType = gtk::Widget;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for SimpleNeedle {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    double_prop("radius"),
                    double_prop("min"),
                    double_prop("max"),
                    integer_prop("start"),
                    integer_prop("end"),
                    double_prop("value"),
                ]
            });
            PROPERTIES.as_ref()
        }
        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "radius" => self.radius.set(value.get().expect("f64 value")),
                "min" => self.min.set(value.get().expect("f64 value")),
                "max" => self.max.set(value.get().expect("f64 value")),
                "start" => self.start.set(value.get().expect("i32 value")),
                "end" => self.end.set(value.get().expect("i32 value")),
                "value" => self.val.set(value.get().expect("f64 value")),
                _ => unimplemented!(),
            }
        }
    }
    impl PaintableImpl for SimpleNeedle {
        fn snapshot(&self, _paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64) {
            println!("=============== needle {} ===============", self.val.get());
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&rect);

            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.paint().unwrap();

            cr.set_source_rgba(0.0, 1.0, 0.0, 1.0);

            let (xc, yc) = (width as f64 / 2.0, height as f64 / 2.0);
            cr.set_line_width(1.0);

            cr.move_to(xc, yc);

            let pct = (self.val.get() - self.min.get()) / (self.max.get() - self.min.get());

            let d = self.end.get() as f64 - ((self.end.get() - self.start.get()) as f64 * pct);
            let a = d * std::f64::consts::PI / 180.0;

            let (x, y) = point_to_arc((xc, yc), a, self.radius.get() * 0.75);
            cr.line_to(x, y);
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
        Self::new(100.0, 0.0, 100.0, 45, 135)
    }
}

impl SimpleNeedle {
    pub fn new(radius: f64, min: f64, max: f64, start: i32, end: i32) -> Self {
        glib::Object::new(&[
            ("radius", &radius),
            ("min", &min),
            ("max", &max),
            ("start", &start),
            ("end", &end),
        ])
        .unwrap()
    }

    pub fn set_radius(&self, val: f64) {
        self.imp().radius.set(val);
        self.notify("radius");
        self.invalidate_contents();
    }

    pub fn set_min(&self, min: f64) {
        self.imp().min.set(min);
        self.notify("min");
        self.invalidate_contents();
    }

    pub fn set_max(&self, max: f64) {
        self.imp().max.set(max);
        self.notify("max");
        self.invalidate_contents();
    }

    pub fn set_start(&self, start: i32) {
        self.imp().start.set(start);
        self.notify("start");
        self.invalidate_contents();
    }

    pub fn set_end(&self, end: i32) {
        self.imp().end.set(end);
        self.notify("end");
        self.invalidate_contents();
    }

    pub fn set_value(&self, val: f64) {
        self.set_property("value", &val);
        self.imp().val.set(val);
        self.invalidate_contents();
    }
}
