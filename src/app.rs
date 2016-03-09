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
    MoveRight
}


impl Component for App {
    type Context = Context;
    type Event = Event;
    type Action = Action;
    type State = i32;
    type View = Element;

    fn intent(&self, context: Context, event: Event) -> Option<Action>
    {
      if event==Press(Keyboard(Key::Right)){Some(Action::MoveRight)}
      else {None}
    }

    fn init(&self) -> i32 {
        0
    }

    fn update(&self, current: i32, action: Action)
        -> i32
    {
        current+1
    }

    fn view(&self, context: Context, state: i32) -> Element {
        let tile_size = 60.0;
        let (width, height) = context.window.size;
        collage(width as i32, height as i32,
                vec![circle(tile_size * 0.5).filled(yellow())
                    .shift_x(state as f64 * tile_size)])
            .clear(black())
    }
}
