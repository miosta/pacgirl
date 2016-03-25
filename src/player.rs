use benzene::Component;
use elmesque::color::yellow;
use elmesque::form::{circle, Form};

pub struct Player;

#[derive(Clone)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown
}


impl Component for Player {
    type Context = ();
    type Action = Action;
    type State = (i32,i32);
    type View = Form;

    fn init(&self) -> (i32,i32) {
        (0, 0)
    }

    fn update(&self, current: (i32,i32), action: Action) -> (i32,i32) {
        match action {
            Action::MoveRight => (current.0 + 1, current.1),
            Action::MoveLeft => (current.0 - 1, current.1),
            Action::MoveUp => (current.0, current.1 + 1),
            Action::MoveDown => (current.0, current.1 - 1),
        }
    }

    fn view(&self, _: (), state: (i32,i32)) -> Form {
        let tile_size = 60.0;
        circle(tile_size * 0.5).filled(yellow())
            .shift_x(state.0 as f64 * tile_size)
            .shift_y(state.1 as f64 * tile_size)
    }
}

