use benzene::Component;
use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use elmesque::Element;
use elmesque::color::{black,yellow};
use elmesque::form::{collage,circle};

pub struct App;

#[derive(Clone)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,

}


impl Component for App {
    type Context = Context;
    type Event = Event;
    type Action = Action;
    type State = (i32,i32);
    type View = Element;

    fn intent(&self, context: Context, event: Event) -> Option<Action> {
        match event {
            Press(Keyboard(Key::Right)) => Some(Action::MoveRight),
            Press(Keyboard(Key::Left)) => Some(Action::MoveLeft),
            Press(Keyboard(Key::Up)) => Some(Action::MoveUp),
            Press(Keyboard(Key::Down)) => Some(Action::MoveDown),
            _ => None
        }
    }

    fn init(&self) -> (i32,i32) {
        (0,0)
    }

    fn update(&self, current: (i32,i32), action: Action) -> (i32,i32) {
        match action {
            Action::MoveRight => (current.0 +1, current.1),
            Action::MoveLeft => (current.0 - 1, current.1),
            Action::MoveUp => (current.0, current.1 + 1),
            Action::MoveDown => (current.0, current.1 - 1),
        }
    }

    fn view(&self, context: Context, state: (i32,i32)) -> Element {
        let tile_size = 60.0;
        let (width, height) = context.window.size;
        collage(width as i32, height as i32,
                vec![circle(tile_size * 0.5).filled(yellow())
                    .shift_x(state.0 as f64 * tile_size)
                    .shift_y(state.1 as f64 * tile_size)
                    ]
                 )
            .clear(black())
    }
}
