#[path = "../../src/gui/lib.rs"]
mod gui;
#[path = "../../src/macos-gui/lib.rs"]
mod macos_gui;
mod render;
#[path = "../../src/window-gui/lib.rs"]
mod windows_gui;
use render::render;

use gui::GuiFactoryDynamic;
use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = false;

    // Allocate a factory object in runtime depending on unpredictable input.
    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    // Factory invocation can be inlined right here.
    let button = factory.create_button();
    button.press();

    // Factory object can be passed to a function as a parameter.
    render(factory);
}
