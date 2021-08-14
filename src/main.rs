use gtk::prelude::*;

mod macros;
mod notebook;
mod settings;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("OpenDragui Mouse");
    window.set_border_width(12);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let mut ntbk = notebook::Notebook::new();

    ntbk.create_tab(
        "General",
        gtk::Label::new(Some("Not yet implemented!")).upcast(),
    );
    ntbk.create_tab(
        "DPI",
        gtk::Label::new(Some("Not yet implemented!")).upcast(),
    );


    let brightness_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 1.0, 9.0, 1.0);
    brightness_scale.set_size_request(200, -1);

    ntbk.create_tab(
        "Lighting",
        settings::settings_view_ui(&vec![
            settings::setting_ui(
                "Lighting mode",
                comboboxtext!(
                    "Breathing",
                    "Rainbow",
                    "Full lighted",
                    "Wave",
                    "Go without trace",
                    "Reactive",
                    "Flash"
                )
                .upcast(),
            ),
            settings::setting_ui(
                "Brightness",
                brightness_scale.upcast(),
            ),
        ]),
    );

    window.add(&ntbk.widget);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.grbavacigla.opendragui"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}
