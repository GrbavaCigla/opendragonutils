use gtk::prelude::*;

pub fn setting_ui(title: &str, widget: gtk::Widget) -> gtk::Widget {
    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    main_box.pack_start(&gtk::Label::new(Some(title)), false, false, 2);
    main_box.pack_end(&widget, false, false, 2);

    main_box.upcast()
}

pub fn settings_view_ui(settings: &Vec<gtk::Widget>) -> gtk::Widget {
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 2);

    for widget in settings.iter() {
        main_box.pack_start(widget, false, false, 2);
    }

    return main_box.upcast();
}