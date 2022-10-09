use yew::prelude::*;

pub struct Card;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub title: Option<String>,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <section class="bg-base-200 shadow rounded p-4 max-w-[min(40ch, fit)] m-4">
                {
                    if let Some(title) = &props.title {
                        html!(
                            <h2 class="text-xl font-bold">{ title }</h2>
                        )
                    } else {
                        html!()
                    }
                }
                { for props.children.iter() }
            </section>
        }
    }
}
