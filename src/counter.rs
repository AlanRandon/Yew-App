use yew::prelude::*;

pub enum Message {
    Add(i64),
}

pub struct Counter {
    value: i64,
}

impl Component for Counter {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    // Returns `true` if a re-render is needed
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add(amount) => {
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
                <button onclick={link.callback(|_| Message::Add(-1))} class="btn">{ "-1" }</button>
                <p class="min-w-[10ch] text-center">{ self.value }</p>
                <button onclick={link.callback(|_| Message::Add(1))} class="btn">{ "+1" }</button>
            </div>
        }
    }
}
