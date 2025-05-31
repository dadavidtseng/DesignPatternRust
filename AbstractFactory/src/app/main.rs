mod render;

#[path = "../../src/gui/lib.rs"]
mod gui;
#[path = "../../src/macos-gui/lib.rs"]
mod macos_gui;
#[path = "../../src/window-gui/lib.rs"]
mod windows_gui;

use render::render;

use crate::macos_gui::factory::MacFactory;
use crate::windows_gui::factory::WindowsFactory;

fn main() {
    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}