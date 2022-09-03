use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseFloatError;
use std::path::{Path, PathBuf};
use std::thread::spawn;
use std::time::Duration;

use gtk::glib::{ObjectExt, StaticTypeExt};
use gtk::{ApplicationWindow, Builder};
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use relm4::{gtk, AppUpdate, Model, RelmApp, Sender, Widgets};

use gauges::layered_gauge::LayeredGauge;

mod simple_gauge;

#[derive(Debug, Default)]
struct AppModel {
    speed: [f64; 9],
}

enum AppMsg {
    Update(usize, f64),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppState;
    type Components = ();
}

struct AppState {
    window: ApplicationWindow,
    watchers: Vec<PollWatcher>,
    gauges: Vec<LayeredGauge>,
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Update(id, speed) => self.speed[id] = speed,
        }
        true
    }
}

impl Widgets<AppModel, ()> for AppState {
    type Root = ApplicationWindow;

    fn init_view(_model: &AppModel, _parent_widgets: &(), _sender: Sender<AppMsg>) -> Self {
        let glade_src = include_str!("dash.glade");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.object("main_window").unwrap();

        let mut watchers = vec![];
        watchers.push(watch(0, "/sys/class/hwmon/hwmon5/fan1_input", &_sender));
        watchers.push(watch(1, "/sys/class/hwmon/hwmon5/fan2_input", &_sender));
        watchers.push(watch(2, "/sys/class/hwmon/hwmon4/temp1_input", &_sender));
        watchers.push(watch(3, "/sys/class/hwmon/hwmon7/temp1_input", &_sender));

        let mut gauges = vec![];
        for i in 0..9 {
            let gauge: LayeredGauge = builder.object(&format!("g{}", i)).unwrap();
            gauges.push(gauge)
        }

        AppState {
            window,
            watchers,
            gauges,
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, _model: &AppModel, _sender: Sender<AppMsg>) {
        for (i, gauge) in self.gauges.iter().enumerate() {
            gauge.set_property("value", _model.speed[i]);
        }
    }
}

fn watch(id: usize, path: &str, sender: &Sender<AppMsg>) -> PollWatcher {
    let config = Config::default()
        .with_compare_contents(true) // crucial part for pseudo filesystems
        .with_poll_interval(Duration::from_secs(1));

    let path = PathBuf::from(path);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = PollWatcher::new(tx, config).unwrap();
    watcher.watch(&path, RecursiveMode::Recursive).unwrap();
    let sender = sender.clone();

    // initialize the view with a reading
    let va: f64 = path_to_reading(&path).unwrap();
    sender.send(AppMsg::Update(id, va)).unwrap();

    // watch for changes and send them over through the sender
    spawn(move || loop {
        for e in rx.recv() {
            match e {
                Ok(_) => {
                    let va: f64 = path_to_reading(&path).unwrap();
                    sender.send(AppMsg::Update(id, va)).unwrap();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    watcher
}

fn path_to_reading(path: &Path) -> Result<f64, ParseFloatError> {
    let mut f = File::open(&path).unwrap();
    let mut line = String::new();
    f.read_to_string(&mut line).unwrap();
    line.trim_end().parse()
}

fn main() -> Result<(), Box<dyn Error>> {
    LayeredGauge::ensure_type();

    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();

    Ok(())
}
