use benzene::Component;
use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;
use player::{Player, Action};

pub struct App {
    player: Player
}

impl App {
    pub fn new() -> App {
        App { player: Player }
    }
}

impl Component for App {
    type Context = Context;
    type Event = Event;
    type Action = Action;
    type State = (i32, i32);
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

    fn init(&self) -> (i32, i32) {
        self.player.init()
    }

    fn update(&self, current: (i32, i32), action: Action) -> (i32,i32) {
        self.player.update(current, action)
    }

    fn view(&self, context: Context, state: (i32,i32)) -> Element {
        let (width, height) = context.window.size;
        collage(width as i32, height as i32,
                vec![self.player.view(context, state)])
            .clear(black())
    }
}
