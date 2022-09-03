use gtk::gdk;
use gtk::glib;
use gtk::glib::ObjectExt;
use gtk::prelude::{PaintableExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp {
    use crate::{double_prop, integer_prop, point_on_arc, uinteger_prop};
    use cairo::{Matrix, MatrixTrait};
    use gtk::gdk::Snapshot;
    use gtk::glib::ffi::G_PI;
    use gtk::glib::{Cast, ParamSpec, ParamSpecDouble, Value};
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, PaintableImpl};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{gdk, glib, graphene};
    use once_cell::sync::Lazy;
    use std::cell::Cell;

    #[derive(Default)]
    pub struct SimpleDial {
        pub radius: Cell<f64>,
        pub start: Cell<i32>,
        pub end: Cell<i32>,
        pub ticks: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleDial {
        const NAME: &'static str = "SimpleDial";
        type Type = super::SimpleDial;
        type ParentType = gtk::Widget;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for SimpleDial {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    double_prop("radius"),
                    integer_prop("start"),
                    integer_prop("end"),
                    uinteger_prop("ticks"),
                ]
            });
            PROPERTIES.as_ref()
        }
        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "radius" => self.radius.set(value.get().expect("f64")),
                "start" => self.start.set(value.get().expect("i32")),
                "end" => self.end.set(value.get().expect("i32")),
                "ticks" => self.ticks.set(value.get().expect("u32")),
                _ => unimplemented!(),
            }
        }
    }
    impl PaintableImpl for SimpleDial {
        fn snapshot(&self, _paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64) {
            if self.radius.get() == 0.0 {
                return;
            }
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&rect);

            // flip the y
            let mat = gtk::cairo::Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, height);
            cr.set_matrix(mat);

            cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            cr.paint().unwrap();

            cr.set_source_rgb(1.0, 1.0, 1.0);

            let (xc, yc) = (width as f64 / 2.0, height as f64 / 2.0);
            cr.set_line_width(1.0);

            // draw center point
            cr.rectangle(xc, yc, 3.0, 3.0);
            cr.stroke().unwrap();

            let theta_start = 180 - self.end.get();
            let theta_end = 180 - self.start.get();
            let radius = self.radius.get();
            cr.arc(
                xc,
                yc,
                radius,
                theta_start as f64 * G_PI / 180.0,
                theta_end as f64 * G_PI / 180.0,
            );
            cr.stroke().unwrap();

            cr.save();
            cr.translate(xc, yc);

            let inset = 10.0;

            cr.set_font_size(18.0);

            let step_angle = ((theta_end - theta_start) / self.ticks.get() as i32)
                .try_into()
                .expect("i32");
            let range = (theta_start..=theta_end);
            for i in range.rev().step_by(step_angle) {
                let inner_tick = point_on_arc(radius - inset, i);
                cr.move_to(inner_tick.0, inner_tick.1);
                let outer_tick = point_on_arc(radius, i);
                cr.line_to(outer_tick.0, outer_tick.1);
                cr.stroke();
            }
            cr.restore();
        }
    }
    impl WidgetImpl for SimpleDial {}
}

glib::wrapper! {
    pub struct SimpleDial(ObjectSubclass<imp::SimpleDial>)
        @extends gtk::Widget,
        @implements gdk::Paintable;
}

impl Default for SimpleDial {
    fn default() -> Self {
        SimpleDial::new(100.0, 45, 135, 10)
    }
}

impl SimpleDial {
    pub fn new(radius: f64, start: i32, end: i32, ticks: u32) -> Self {
        glib::Object::new(&[
            ("radius", &radius),
            ("start", &start),
            ("end", &end),
            ("ticks", &ticks),
        ])
        .unwrap()
    }

    pub fn set_radius(&self, radius: f64) {
        self.imp().radius.set(radius);
        self.notify("radius");
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

    pub fn set_ticks(&self, ticks: u32) {
        self.imp().ticks.set(ticks);
        self.notify("ticks");
        self.invalidate_contents();
    }
}
