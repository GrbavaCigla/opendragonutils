use std::{cell::RefCell, rc::Rc};

use gtk::prelude::*;

#[derive(Debug)]
pub struct Notebook {
    pub widget: gtk::Box,

    button_box: gtk::Box,
    notebook: gtk::Notebook,
    buttons: Rc<RefCell<Vec<gtk::ToggleButton>>>,
}

impl Notebook {
    pub fn new() -> Self {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        let notebook = gtk::Notebook::new();
        notebook.set_show_tabs(false);

        main_box.pack_start(&button_box, false, false, 2);
        main_box.pack_start(&notebook, true, true, 2);

        Self {
            widget: main_box,
            button_box: button_box,
            notebook: notebook,
            buttons: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn create_tab(&mut self, title: &str, widget: gtk::Widget) {
        let button = gtk::ToggleButton::with_label(title);
        self.button_box.pack_start(&button, true, true, 2);

        let buttons = Rc::clone(&self.buttons);
        button.connect_toggled(move |x| {
            for i in buttons.borrow().iter() {
                i.set_active(false);
            }
            x.set_active(true);
        });
        println!("{:#?}", self.buttons);
        
        self.buttons.borrow_mut().push(button);

        self.notebook
        .append_page(&widget, Some(&gtk::Label::new(Some(title))));
        
    }
}
