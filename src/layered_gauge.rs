use gtk::glib;

mod imp {
    use crate::parts;
    use gtk::builders::OverlayBuilder;
    use gtk::prelude::WidgetExt;
    use gtk::subclass::prelude::{
        ObjectImpl, ObjectImplExt, ObjectSubclass, WidgetClassSubclassExt,
    };
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{glib, Picture};
    use std::cell::Cell;

    #[derive(Default)]
    pub struct LayeredGauge {
        pub width: Cell<i32>,
        pub height: Cell<i32>,
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

            println!("-------- layers constructed --------");

            let overlay = OverlayBuilder::new().build();
            overlay.set_parent(obj);

            let dial = parts::simple_dial::SimpleDial::new();
            let picture = Picture::builder().paintable(&dial).build();
            overlay.add_overlay(&picture);

            let needle = parts::simple_needle::SimpleNeedle::new();
            let picture = Picture::builder().paintable(&needle).build();
            overlay.add_overlay(&picture);
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
        Self::new()
    }
}

impl LayeredGauge {
    pub fn new() -> Self {
        let paintable: Self = glib::Object::new(&[]).unwrap();
        paintable
    }
}
