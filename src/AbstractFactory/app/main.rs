mod render;

#[path = "../../AbstractFactory/macos-gui/lib.rs"]
mod macos_gui;
#[path = "../../AbstractFactory/window-gui/lib.rs"]
mod windows_gui;

use render::render;

use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}