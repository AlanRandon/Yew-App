#![warn(clippy::pedantic)]

use yew::prelude::*;

mod counter;
mod navbar;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    // Returns `true` if a re-render is needed
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <navbar::Navbar />
                <main class="p-4 grid justify-center">
                    <counter::Counter />
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
