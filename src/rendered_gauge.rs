use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{cairo, gdk, glib, graphene};

use gtk::glib::ffi::G_PI;
use rsvg::SvgHandle;
use std::cell::Cell;
use std::path::PathBuf;

mod imp {
    use super::*;

    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    #[derive(Default)]
    pub struct RenderedGauge {
        pub handle: OnceCell<Vec<rsvg::SvgHandle>>,
        pub scale_factor: Cell<i32>,
        pub width: Cell<i32>,
        pub height: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RenderedGauge {
        const NAME: &'static str = "RenderedGauge";
        type Type = super::RenderedGauge;
        type ParentType = glib::Object;
        type Interfaces = (gdk::Paintable,);

        fn new() -> Self {
            Self {
                handle: OnceCell::new(),
                scale_factor: Cell::new(1),
                width: Cell::new(0),
                height: Cell::new(0),
            }
        }
    }

    impl ObjectImpl for RenderedGauge {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpecInt::new(
                    "scale-factor",
                    "scale-factor",
                    "scale-factor",
                    0,
                    i32::MAX,
                    1,
                    glib::ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "scale-factor" => self.scale_factor.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "scale-factor" => {
                    self.scale_factor.set(value.get().unwrap());
                    obj.invalidate_contents();
                }
                _ => unimplemented!(),
            };
        }
    }
    impl PaintableImpl for RenderedGauge {
        fn snapshot(
            &self,
            paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let rect = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&rect);

            paintable
                .render(&cr, width as i32, height as i32)
                .unwrap_or_else(|err| eprintln!("Failed to render svg: {:?}", err));
        }

        fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
            let handle = self.handle.get().unwrap().first().unwrap();
            let renderer = rsvg::CairoRenderer::new(handle);

            renderer.intrinsic_dimensions().width.length as i32
        }

        fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
            let renderer = rsvg::CairoRenderer::new(self.handle.get().unwrap().first().unwrap());

            renderer.intrinsic_dimensions().height.length as i32
        }
    }

    impl WidgetImpl for RenderedGauge {}
    impl RangeImpl for RenderedGauge {}
}

glib::wrapper! {
    pub struct RenderedGauge(ObjectSubclass<imp::RenderedGauge>)
        @extends gtk::Widget, gtk::Range,
        @implements gdk::Paintable;
}

impl Default for RenderedGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderedGauge {
    pub fn new() -> Self {
        let paintable: Self = glib::Object::new(&[]).unwrap();
        let parts = vec![
            "resources/clock-drop-shadow.svg",
            "resources/clock-drop-shadow.svg",
            "resources/clock-face.svg",
            "resources/clock-marks.svg",
            "resources/clock-minute-hand-shadow.svg",
            "resources/clock-minute-hand.svg",
            "resources/clock-face-shadow.svg",
            "resources/clock-glass.svg",
            "resources/clock-frame.svg",
        ];

        let parts: Vec<SvgHandle> = parts
            .iter()
            .map(PathBuf::from)
            .map(|p| rsvg::Loader::new().read_path(p).unwrap())
            .collect();

        paintable
            .imp()
            .handle
            .set(parts)
            .unwrap_or_else(|_| eprintln!("Could not set handle"));

        paintable
    }

    pub fn render(&self, cr: &cairo::Context, width: i32, height: i32) -> anyhow::Result<()> {
        let imp = self.imp();
        let renderers: Vec<rsvg::CairoRenderer> = imp
            .handle
            .get()
            .unwrap()
            .into_iter()
            .map(|h| rsvg::CairoRenderer::new(h))
            .collect();

        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height)?;
        let cr2 = cairo::Context::new(&surface)?;

        for renderer in renderers.iter().take(5) {
            renderer.render_document(
                &cr2,
                &cairo::Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: width as f64,
                    height: height as f64,
                },
            )?;
        }

        let lower = -4000.0;
        let upper = 4000.0;
        let adjustment_get_value = 2000.0;
        let angle = adjustment_get_value * 2.0 * G_PI / (upper - lower);

        let angle = -angle;

        println!("angle: {}", angle);

        // 1
        cr2.save()?;

        cr2.translate(width as f64 / 2.0, height as f64 / 2.0);
        cr2.save()?;

        //cr2.translate(-0.75, 0.75);
        cr2.rotate(angle);

        renderers[4].render_document(
            &cr2,
            &cairo::Rectangle {
                x: 0.0,
                y: 0.0,
                width: width as f64,
                height: height as f64,
            },
        )?;

        cr2.restore()?;
        cr2.rotate(angle);

        renderers[5].render_document(
            &cr2,
            &cairo::Rectangle {
                x: 0.0,
                y: 0.0,
                width: width as f64,
                height: height as f64,
            },
        )?;

        cr2.restore()?;

        for renderer in renderers.iter().skip(4 + 2) {
            renderer.render_document(
                &cr2,
                &cairo::Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: width as f64,
                    height: height as f64,
                },
            )?;
        }

        //cr.scale(1.0 / scale as f64, 1.0 / scale as f64);
        cr.set_source_surface(&surface, 0.0, 0.0)?;
        cr.paint()?;

        Ok(())
    }
}
