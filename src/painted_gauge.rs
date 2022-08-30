use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{cairo, gdk, gio, glib, graphene};

use gtk::gio::Cancellable;
use rsvg::SvgHandle;
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

mod imp {
    use super::*;

    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    #[derive(Default)]
    pub struct PaintedGauge {
        pub handle: OnceCell<Vec<rsvg::SvgHandle>>,
        pub scale_factor: Cell<i32>,
        pub width: Cell<i32>,
        pub height: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaintedGauge {
        const NAME: &'static str = "PaintedGauge";
        type Type = super::PaintedGauge;
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

    impl ObjectImpl for PaintedGauge {
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
    impl PaintableImpl for PaintedGauge {
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
}

glib::wrapper! {
    pub struct PaintedGauge(ObjectSubclass<imp::PaintedGauge>)
        @implements gdk::Paintable;
}

impl Default for PaintedGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl PaintedGauge {
    pub fn new() -> Self {
        let paintable: Self = glib::Object::new(&[]).unwrap();
        let parts = vec![
            "resources/clock-drop-shadow.svg",
            "resources/clock-drop-shadow.svg",
            "resources/clock-face.svg",
            "resources/clock-marks.svg",
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

        // FIXME Remove if https://gitlab.gnome.org/GNOME/librsvg/-/issues/826 is fixed
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height)?;
        let cr2 = cairo::Context::new(&surface)?;

        for renderer in renderers {
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
