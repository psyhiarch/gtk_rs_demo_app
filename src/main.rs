use gtk::{gio, glib, prelude::*};

use crate::window::Window;

mod window;

const APP_ID: &'static str = "org.gtk_rs.GtkDemoApp";

fn main() -> glib::ExitCode {
    // Load resources from installed location
    let res = gio::Resource::load("build/src/resources/resources.gresource")
        .expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
