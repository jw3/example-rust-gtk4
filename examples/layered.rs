use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::spawn;
use std::time::Duration;

use gtk::glib::{ObjectExt, StaticTypeExt};
use gtk::prelude::GtkWindowExt;
use gtk::ApplicationWindow;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use relm4::{gtk, AppUpdate, Model, RelmApp, Sender, Widgets};

use gauges::layered_gauge::LayeredGauge;
use gauges::parts::simple_dial::SimpleDial;

#[derive(Debug, Default)]
struct AppModel {
    speed: f64,
}

enum AppMsg {
    Update(f64),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppState;
    type Components = ();
}

struct AppState {
    window: ApplicationWindow,
    _watcher: PollWatcher,
    gauge: LayeredGauge,
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Update(speed) => self.speed = speed,
        }
        true
    }
}

impl Widgets<AppModel, ()> for AppState {
    type Root = ApplicationWindow;

    fn init_view(_model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = ApplicationWindow::builder()
            .default_width(600)
            .default_height(600)
            .build();

        let gauge = LayeredGauge::new(100.0, 0.0, 6000.0, 45, 135, 6);
        window.set_child(Some(&gauge));

        let (tx, rx) = std::sync::mpsc::channel();
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_millis(250));

        let path = Path::new("/sys/class/hwmon/hwmon5/fan1_input");
        let mut watcher = PollWatcher::new(tx, config).unwrap();
        watcher.watch(&path, RecursiveMode::Recursive).unwrap();

        // map the rx in the model over to the sender
        spawn(move || loop {
            for e in rx.recv() {
                match e {
                    Ok(_) => {
                        let mut f = File::open(path).unwrap();
                        let mut line = String::new();
                        f.read_to_string(&mut line).unwrap();
                        let va: f64 = line.trim_end().parse().unwrap();
                        sender.send(AppMsg::Update(va)).unwrap();
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });

        AppState {
            window,
            _watcher: watcher,
            gauge,
        }
    }
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }
    fn view(&mut self, _model: &AppModel, _sender: Sender<AppMsg>) {
        self.gauge.set_property("value", _model.speed);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleDial::ensure_type();

    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();

    Ok(())
}
