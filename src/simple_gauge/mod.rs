mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct SimpleGauge(ObjectSubclass<imp::SimpleGauge>)
        @extends gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for SimpleGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleGauge {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("gobject creation")
    }
}
