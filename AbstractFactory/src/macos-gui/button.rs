﻿use crate::gui::Button;

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("Mac button has pressed");
    }
}