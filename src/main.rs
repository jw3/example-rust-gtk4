use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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
    gauges: Vec<LayeredGauge>,
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
        let glade_src = include_str!("dash.glade");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.object("main_window").unwrap();

        let (tx, rx) = std::sync::mpsc::channel();
        let config = Config::default()
            .with_compare_contents(true) // crucial part for pseudo filesystems
            .with_poll_interval(Duration::from_secs(1));

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

        let mut gauges = vec![];
        for i in 0..9 {
            let gauge: LayeredGauge = builder.object(&format!("g{}", i)).unwrap();
            gauges.push(gauge)
        }

        AppState {
            window,
            _watcher: watcher,
            gauges,
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, _model: &AppModel, _sender: Sender<AppMsg>) {
        for gauge in &self.gauges {
            gauge.set_property("value", _model.speed);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    LayeredGauge::ensure_type();

    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();

    Ok(())
}
