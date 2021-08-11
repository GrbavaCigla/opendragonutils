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
    ntbk.create_tab("blaaaaaaa", gtk::Label::new(Some("blaaaaaa")).upcast());
    ntbk.create_tab("blaaaaaaa", gtk::Label::new(Some("blaaaaaa")).upcast());
    window.add(&ntbk.widget);

    // let notebook = gtk::Notebook::new();
    // notebook.set_show_tabs(false);
    // notebook.append_page(
    //     &gtk::Label::new(Some("Not yet available!")),
    //     Some(&gtk::Label::new(Some("General"))),
    // );
    // notebook.append_page(
    //     &gtk::Label::new(Some("Not yet available!")),
    //     Some(&gtk::Label::new(Some("Light"))),
    // );

    // let tab_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    // tab_box.pack_start(&gtk::ToggleButton::with_label("Light"), true, true, 0);
    // tab_box.pack_start(&gtk::ToggleButton::with_label("Light"), true, true, 0);
    // tab_box.pack_start(&gtk::ToggleButton::with_label("Light"), true, true, 0);
    
    // let main_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    // main_box.pack_start(&tab_box, false, false, 0);
    // main_box.pack_start(&notebook, true, true, 0);

    // window.add(&main_box);
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
