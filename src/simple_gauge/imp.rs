use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct SimpleGauge {
    child: RefCell<Option<gtk::Widget>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleGauge {
    const NAME: &'static str = "SimpleGauge";
    type Type = super::SimpleGauge;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for SimpleGauge {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        // Create the child label.
        let child = gtk::Scale::default();
        child.set_range(0.0, 2400.0);
        child.set_value(810.0);
        child.set_parent(obj);
        child.set_draw_value(true);
        *self.child.borrow_mut() = Some(child.upcast::<gtk::Widget>());
    }

    fn dispose(&self, _obj: &Self::Type) {
        // Child widgets need to be manually unparented in `dispose()`.
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SimpleGauge {}
