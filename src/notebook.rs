use std::{cell::RefCell, rc::Rc, sync::Arc};

use gtk::prelude::*;

#[derive(Debug)]
pub struct Notebook {
    pub widget: gtk::Box,

    button_box: gtk::Box,
    notebook: Arc<gtk::Notebook>,
    buttons: Rc<RefCell<Vec<gtk::ToggleButton>>>,

}

impl Notebook {
    pub fn new() -> Self {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        let notebook = Arc::new(gtk::Notebook::new());
        notebook.set_show_tabs(false);
        notebook.set_show_border(false);

        main_box.pack_start(&button_box, false, false, 2);
        main_box.pack_start(&*notebook, true, true, 2);

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
        let notebook = self.notebook.clone();
        button.connect_clicked(move |x| {
            let mut c = 0;
            for (i, btn) in buttons.borrow().iter().enumerate() {
                if x == btn {
                    c = i;
                } else {
                    btn.set_active(false);
                }
            }
            notebook.set_current_page(Some(c as u32));
        });
        
        self.buttons.borrow_mut().push(button);

        self.notebook
        .append_page(&widget, Some(&gtk::Label::new(Some(title))));
        
    }
}
