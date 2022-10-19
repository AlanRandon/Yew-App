use yew::prelude::*;

#[derive(PartialEq, Eq)]
pub enum BtnShape {
    Close,
    Hamburger,
}

impl BtnShape {
    fn is_hamburger(&self) -> bool {
        self == &BtnShape::Hamburger
    }
}

pub struct Hamburger {
    btn_shape: BtnShape,
}

pub enum Message {
    Toggle,
}

impl Component for Hamburger {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            btn_shape: BtnShape::Hamburger,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Toggle => {
                if self.btn_shape.is_hamburger() {
                    self.btn_shape = BtnShape::Close;
                } else {
                    self.btn_shape = BtnShape::Hamburger;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let is_close = !self.btn_shape.is_hamburger();

        html! {
            <button onclick={link.callback(|_| Message::Toggle)}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 100 100" stroke-width="5" stroke="currentColor" class="w-6 h-6">
                    <g
                        class={classes!(
                            is_close.then(|| "rotate-45"),
                            "origin-center",
                            "ease-[cubic-bezier(0.0,0.05,0.795,0.035)]"
                        )}
                    >
                        <line
                            x1="5"
                            y1="25"
                            x2="95"
                            y2="25"
                            stroke-linecap="round"
                            class={classes!(
                                is_close.then(|| "translate-y-[25px]"),
                                "transition"
                            )}
                        />
                        <line
                            x1="5"
                            y1="50"
                            x2="95"
                            y2="50"
                            stroke-linecap="round"
                            class={classes!(
                                is_close
                                    .then(|| vec!["rotate-90"]),
                                "transition",
                                "origin-center"
                            )}
                        />
                        <line
                            x1="5"
                            y1="75"
                            x2="95"
                            y2="75"
                            stroke-linecap="round"
                            class={classes!(
                                is_close
                                    .then(|| vec!["-translate-y-[25px]"]),
                                "transition"
                            )}
                        />
                    </g>
                </svg>
            </button>
        }
    }
}
