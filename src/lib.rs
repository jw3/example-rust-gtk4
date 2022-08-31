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
