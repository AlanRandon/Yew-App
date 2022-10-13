use yew::prelude::*;

pub struct Card;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub title: Option<String>,

    #[prop_or_default]
    pub image: Option<String>,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let title = if let Some(ref title) = props.title {
            html!(
                <h2 class="text-xl font-bold">{ title }</h2>
            )
        } else {
            html!()
        };
        let image_alt = props.title.clone().unwrap_or_default();
        let image = if let Some(ref source) = props.image {
            html!(
                <div class="relative">
                    <div class="gradient-mask pointer-events-none"></div>
                    <img src={source.clone()} class="h-32 w-full object-cover object-bottom bg-black" alt={image_alt} />
                </div>
            )
        } else {
            html!()
        };
        html! {
            <section class="bg-base-200 shadow rounded max-w-40ch m-4 w-full">
                { image }
                <div class="p-4 text-black/70">
                    { title }
                    { for props.children.iter() }
                </div>
            </section>
        }
    }
}
