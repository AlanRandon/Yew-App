use yew::prelude::*;

enum Msg {
    Add(i64),
}

struct Counter {
    value: i64,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    // Returns `true` if a re-render is needed
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(amount) => {
                self.value += amount;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // `link()` gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="block flex flex-row items-center gap-4">
                <button onclick={link.callback(|_| Msg::Add(-1))} class="btn">{ "-1" }</button>
                <p>{ self.value }</p>
                <button onclick={link.callback(|_| Msg::Add(1))} class="btn">{ "+1" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Counter>();
}
