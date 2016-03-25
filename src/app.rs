use benzene::{Application, Component};
use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;
use player::{Player, Action as PlayerAction};

pub struct App {
    player: Player
}

impl App {
    pub fn new() -> App {
        App { player: Player }
    }
}

fn bindings(wasd: [Key; 4], key: Key) -> Option<PlayerAction> {
    if wasd[0] == key {
        Some(PlayerAction::MoveUp)
    } else if wasd[1] == key {
        Some(PlayerAction::MoveLeft)
    } else if wasd[2] == key {
        Some(PlayerAction::MoveDown)
    } else if wasd[3] == key {
        Some(PlayerAction::MoveRight)
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
    action: PlayerAction
}

impl Application for App {
    type Event = Event;

    fn intent(&self, _: Context, event: Event) -> Option<Action> {
        if let Press(Keyboard(key)) = event {
            action(0, [Key::Up, Key::Left, Key::Down, Key::Right], key)
            .or(action(1, [Key::W, Key::A, Key::S, Key::D], key))
        } else {
            None
        }
    }
}

impl Component for App {
    type Context = Context;
    type Action = Action;
    type State = [(i32, i32); 2];
    type View = Element;

    fn init(&self) -> [(i32, i32); 2] {
        [self.player.init(), self.player.init()]
    }

    fn update(&self, current: [(i32, i32); 2], action: Action) -> [(i32, i32); 2] {
        match action.player {
            0 => [self.player.update(current[0], action.action), current[1]],
            1 => [current[0], self.player.update(current[1], action.action)],
            _ => current
        }
    }

    fn view(&self, context: Context, state: [(i32, i32); 2]) -> Element {
        let (width, height) = context.window.size;
        collage(width as i32, height as i32,
                vec![
                self.player.view((), state[0]),
                self.player.view((), state[1])
                    ]
                )
            .clear(black())
    }
}
