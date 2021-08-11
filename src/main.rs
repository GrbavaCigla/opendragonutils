use gtk::prelude::*;

mod notebook;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("OpenDragui");
    window.set_border_width(12);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let mut ntbk = notebook::Notebook::new();
    ntbk.create_tab("blaaaaaaa", gtk::Label::new(Some("blaaaaaa")).upcast());
    ntbk.create_tab("blaaaaaaa", gtk::Label::new(Some("fooooooo")).upcast());
    ntbk.create_tab("blaaaaaaa", gtk::Label::new(Some("baaaaaaz")).upcast());
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