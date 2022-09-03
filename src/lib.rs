use gtk::glib::ffi::G_PI;
use gtk::glib::{ParamSpec, ParamSpecDouble, ParamSpecInt, ParamSpecUInt};

pub mod layered_gauge;
pub mod rendered_gauge;
pub mod simple_gauge;

pub mod parts;

#[no_mangle]
pub extern "C" fn gauges_catalog_init() {
    // SimpleGauge::ensure_type();
}

// #[no_mangle]
// pub extern "C" fn simple_gauge_get_type() -> GType {
//     SimpleGauge::ensure_type();
//     SimpleGauge::static_type().into_glib()
// }

pub(crate) fn uinteger_prop(name: &str) -> ParamSpec {
    ParamSpecUInt::builder(name).build()
}

pub(crate) fn integer_prop(name: &str) -> ParamSpec {
    ParamSpecInt::builder(name).build()
}

pub(crate) fn double_prop(name: &str) -> ParamSpec {
    ParamSpecDouble::builder(name).build()
}

pub fn point_on_arc(r: f64, i: i32) -> (f64, f64) {
    (
        r * (i as f64 * G_PI / 180.0).cos(),
        r * (i as f64 * G_PI / 180.0).sin(),
    )
}

pub fn point_to_arc(from: (f64, f64), a: f64, len: f64) -> (f64, f64) {
    (from.0 + (len * a.cos()), from.1 - (len * a.sin()))
}
