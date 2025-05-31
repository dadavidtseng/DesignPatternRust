use crate::gui::Checkbox;

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("Mac checkbox has switched");
    }
}