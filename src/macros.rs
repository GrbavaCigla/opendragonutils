#[macro_export]
macro_rules! comboboxtext {
    ( $( $x:expr ),* ) => {
        {
            let combo = gtk::ComboBoxText::new();
            $(
                combo.append_text($x);
            )*
            combo.set_active(Some(0));
            combo
        }
    };
}