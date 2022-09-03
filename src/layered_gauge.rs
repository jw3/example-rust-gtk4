use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp {
    use crate::parts::simple_dial::SimpleDial;
    use crate::parts::simple_needle::SimpleNeedle;
    use crate::{double_prop, integer_prop, parts, uinteger_prop};
    use gtk::builders::OverlayBuilder;
    use gtk::glib::subclass::InitializingObject;
    use gtk::glib::{ObjectExt, ParamSpec, ParamSpecDouble, ToValue, Value};
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::{
        ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassIsExt, WidgetClassSubclassExt,
    };
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{glib, Picture};
    use once_cell::sync::Lazy;
    use std::cell::{Cell, RefCell};

    #[derive(Default)]
    pub struct LayeredGauge {
        pub radius: Cell<f64>,
        pub min: Cell<f64>,
        pub max: Cell<f64>,
        pub start: Cell<i32>,
        pub end: Cell<i32>,
        pub ticks: Cell<u32>,
        pub dial: RefCell<SimpleDial>,
        pub needle: RefCell<SimpleNeedle>,
        pub val: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LayeredGauge {
        const NAME: &'static str = "LayeredGauge";
        type Type = super::LayeredGauge;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for LayeredGauge {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let overlay = OverlayBuilder::new().build();
            overlay.set_parent(obj);

            *self.dial.borrow_mut() = SimpleDial::new(
                obj.property("radius"),
                obj.property("start"),
                obj.property("end"),
                obj.property("ticks"),
            );
            let picture = Picture::builder().paintable(&*self.dial.borrow()).build();
            overlay.add_overlay(&picture);

            *self.needle.borrow_mut() = SimpleNeedle::new(
                obj.property("radius"),
                obj.property("min"),
                obj.property("max"),
                obj.property("start"),
                obj.property("end"),
            );
            let picture = Picture::builder().paintable(&*self.needle.borrow()).build();
            overlay.add_overlay(&picture);
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    double_prop("radius"),
                    double_prop("min"),
                    double_prop("max"),
                    integer_prop("start"),
                    integer_prop("end"),
                    uinteger_prop("ticks"),
                    double_prop("value"),
                ]
            });
            PROPERTIES.as_ref()
        }
        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "radius" => {
                    let radius = value.get().expect("f64");
                    self.radius.set(radius);
                    self.dial.borrow_mut().set_radius(radius);
                    self.needle.borrow_mut().set_radius(radius);
                }
                "min" => {
                    let min = value.get().expect("f64");
                    self.min.set(min);
                    self.needle.borrow_mut().set_min(min);
                }
                "max" => {
                    let max = value.get().expect("f64");
                    self.max.set(max);
                    self.needle.borrow_mut().set_max(max);
                }
                "start" => {
                    let start = value.get().expect("i32");
                    self.start.set(start);
                    self.dial.borrow_mut().set_start(start);
                    self.needle.borrow_mut().set_start(start);
                }
                "end" => {
                    let end = value.get().expect("i32");
                    self.end.set(end);
                    self.dial.borrow_mut().set_end(end);
                    self.needle.borrow_mut().set_end(end);
                }
                "ticks" => {
                    let ticks = value.get().expect("u32");
                    self.ticks.set(ticks);
                    self.dial.borrow_mut().set_ticks(ticks);
                }
                "value" => self
                    .needle
                    .borrow_mut()
                    .set_value(value.get().expect("f64")),
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "radius" => self.radius.get().to_value(),
                "min" => self.min.get().to_value(),
                "max" => self.max.get().to_value(),
                "start" => self.start.get().to_value(),
                "end" => self.end.get().to_value(),
                "ticks" => self.ticks.get().to_value(),
                "value" => self.val.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for LayeredGauge {}
}

glib::wrapper! {
    pub struct LayeredGauge(ObjectSubclass<imp::LayeredGauge>)
        @extends gtk::Widget;
}

impl Default for LayeredGauge {
    fn default() -> Self {
        Self::new(100.0, 0.0, 100.0, 45, 135, 10)
    }
}

impl LayeredGauge {
    pub fn new(radius: f64, min: f64, max: f64, start: i32, end: i32, ticks: u32) -> Self {
        glib::Object::new(&[
            ("radius", &radius),
            ("min", &min),
            ("max", &max),
            ("start", &start),
            ("end", &end),
            ("ticks", &ticks),
        ])
        .unwrap()
    }
}
