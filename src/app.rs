use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;
use player;


fn bindings(wasd: [Key; 4], key: Key) -> Option<player::Action> {
    use player::Action::*;

    if wasd[0] == key {
        Some(MoveUp)
    } else if wasd[1] == key {
        Some(MoveLeft)
    } else if wasd[2] == key {
        Some(MoveDown)
    } else if wasd[3] == key {
        Some(MoveRight)
    } else {
        None
    }
}

fn action(player: u8, wasd: [Key; 4], key: Key) -> Option<Action> {
    bindings(wasd, key).map(|a| Action { action: a, player: player})
}

#[derive(Clone)]
pub struct Action {
    player: u8,
    action: player::Action
}

pub fn intent(_: Context, event: Event) -> Option<Action> {
    if let Press(Keyboard(key)) = event {
        action(0, [Key::Up, Key::Left, Key::Down, Key::Right], key)
        .or(action(1, [Key::W, Key::A, Key::S, Key::D], key))
    } else {
        None
    }
}

pub type State = [(i32, i32); 2];
pub type View = Element;

pub fn init() -> [(i32, i32); 2] {
    [player::init(), player::init()]
}

pub fn update(current: [(i32, i32); 2], action: Action) -> State {
    match action.player {
        0 => [player::update(current[0], action.action), current[1]],
        1 => [current[0], player::update(current[1], action.action)],
        _ => current
    }
}

pub fn view(context: Context, state: [(i32, i32); 2]) -> View {
    let (width, height) = context.window.size;
    collage(width as i32, height as i32,
            vec![player::view((), state[0]), player::view((), state[1])])
        .clear(black())
}
