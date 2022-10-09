#![warn(clippy::pedantic)]

use card::Card;
use counter::Counter;
use navbar::Navbar;
use yew::prelude::*;

mod card;
mod counter;
mod navbar;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Navbar />
                <main class="flex flex-col items-center gap-4 w-full p-4">
                    <div class="card-container">
                        <Card title="Lorem Ipsum">
                            { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Fugiat consectetur cupiditate, in itaque eum iste expedita asperiores similique. Aperiam, ad provident? Accusamus repudiandae aliquam dolore totam. Consequuntur, similique aperiam? Explicabo?" }
                        </Card>
                        <Card title="Lorem Ipsum">
                            { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Fugiat consectetur cupiditate, in itaque eum iste expedita asperiores similique. Aperiam, ad provident? Accusamus repudiandae aliquam dolore totam. Consequuntur, similique aperiam? Explicabo?" }
                        </Card>
                        <Card title="Lorem Ipsum">
                            { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Fugiat consectetur cupiditate, in itaque eum iste expedita asperiores similique. Aperiam, ad provident? Accusamus repudiandae aliquam dolore totam. Consequuntur, similique aperiam? Explicabo?" }
                        </Card>
                    </div>
                    <Counter />
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
