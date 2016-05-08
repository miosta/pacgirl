extern crate elmesque;
extern crate piston;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;
extern crate benzene;
extern crate benzene_2d;
extern crate input;

use piston::window::WindowSettings;
use benzene::{Driver, Component, interpret, start};
use benzene_2d::Driver2d;

mod app;
mod player;

fn settings() -> WindowSettings {
    WindowSettings::new("Pacfriends", (800, 600))
}

fn main() {
    let mut driver2d = Driver2d::new(settings());
    let output = start(
        Component {
            init: app::init(),
            update: app::update,
            view: app::view,
            effect: |_, _| None
        },
        interpret(driver2d.output(), app::intent)
    );
    driver2d.run(output);
}
