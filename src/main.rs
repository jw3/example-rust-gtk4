use gtk::glib::StaticTypeExt;
use gtk::{ApplicationWindow, Builder};
use relm4::{gtk, AppUpdate, Model, RelmApp, Sender, Widgets};

use crate::simple_gauge::SimpleGauge;

mod simple_gauge;

#[derive(Default)]
struct AppModel {
    speed: u8,
}

enum AppMsg {
    Update(u8),
    Stopped,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Update(speed) => {
                self.speed = speed;
            }
            AppMsg::Stopped => {
                self.speed = 0;
            }
        }
        true
    }
}

struct AppWidgets {
    window: ApplicationWindow,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = ApplicationWindow;

    fn init_view(_model: &AppModel, _parent_widgets: &(), _sender: Sender<AppMsg>) -> Self {
        let glade_src = include_str!("gauge.glade");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.object("main_window").unwrap();

        AppWidgets { window }
    }
    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }
    /// Update the view to represent the updated model.
    fn view(&mut self, _model: &AppModel, _sender: Sender<AppMsg>) {
        //self.label.set_label(&format!("Counter: {}", model.counter));
    }
}

fn main() {
    SimpleGauge::ensure_type();

    let model = AppModel::default();
    let app = RelmApp::new(model);

    app.run();
}
