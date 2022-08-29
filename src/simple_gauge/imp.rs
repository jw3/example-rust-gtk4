use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{ParamSpec, ParamSpecDouble, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Debug, Default)]
pub struct SimpleGauge {
    child: gtk::Scale,
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleGauge {
    const NAME: &'static str = "SimpleGauge";
    type Type = super::SimpleGauge;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for SimpleGauge {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecDouble::builder("gauge-value").build()]);
        PROPERTIES.as_ref()
    }
    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "gauge-value" => {
                let range = &self.child;
                let input_number = value.get().expect("f64 value");
                range.set_value(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        // Create the child label.
        let child = &self.child;
        child.set_range(0.0, 6000.0);
        child.set_parent(obj);
        child.set_draw_value(true);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.child.unparent();
    }
}

impl WidgetImpl for SimpleGauge {}
