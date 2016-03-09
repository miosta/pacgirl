use benzene::Component;
use carboxyl_window::{Context, Event};
use elmesque::Element;
use elmesque::color::{black,yellow};
use elmesque::form::{collage,circle};

pub struct App;

impl Component for App {
    type Context = Context;
    type Event = Event;
    type Action = ();
    type State = ();
    type View = Element;

    fn intent(&self, context: Context, event: Event) -> Option<()>
    {
      None
    }

    fn init(&self) -> () {
       ()
    }

    fn update(&self, current: (), action: ())
        -> ()
    {
        ()
    }

    fn view(&self, context: Context, state: ()) -> Element {
        
        let (width, height) = context.window.size;
        collage(width as i32, height as i32, vec![circle(30.0).filled(yellow())])
            .clear(black())
    }
}
