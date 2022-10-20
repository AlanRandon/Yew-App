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

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub btn_shape: BtnShape,
}

pub struct Hamburger;

impl Component for Hamburger {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    // fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    //     false
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_close = !ctx.props().btn_shape.is_hamburger();

        html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 100 100" stroke-width="5" stroke="currentColor" class="w-6 h-6 hover:opacity-70 transition">
                <g
                    class={classes!(
                        is_close.then(|| vec!["rotate-45", "delay-100"]),
                        "origin-center",
                        "transition"
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
                                .then(|| vec!["rotate-90", "delay-200"]),
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
        }
    }
}
