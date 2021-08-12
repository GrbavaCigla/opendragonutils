use gtk::prelude::*;

mod notebook;

fn light_mode_ui() -> gtk::Widget {
    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    
    return main_box.upcast();
}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("OpenDragui Mouse");
    window.set_border_width(12);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let mut ntbk = notebook::Notebook::new();
    ntbk.create_tab("General", gtk::Label::new(Some("Not yet implemented!")).upcast());
    ntbk.create_tab("DPI", gtk::Label::new(Some("Not yet implemented!")).upcast());
    ntbk.create_tab("Lighting", light_mode_ui());
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