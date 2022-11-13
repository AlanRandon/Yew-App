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

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub action_elements: Html,
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
                <h2 class="card-title">{ title }</h2>
            )
        } else {
            html!()
        };
        let image_alt = props.title.clone().unwrap_or_default();
        let image = if let Some(ref source) = props.image {
            html!(
                <figure class="relative h-min">
                    <div class="gradient-mask pointer-events-none"></div>
                    <img src={source.clone()} class="h-32 w-full object-cover object-bottom bg-black" alt={image_alt} />
                </figure>
            )
        } else {
            html!()
        };
        html! {
            <section class={classes!(
                "card",
                "bg-base-200",
                "min-w-full",
                "shadow",
                props.classes.clone()
            )}>
                { image }
                <div class="text-black/70 p-4 grow">
                    { title }
                    { for props.children.iter() }
                </div>

                {
                    if props.action_elements == html!() {
                        html!()
                    } else {
                        html!(
                            <div class="card-actions justify-end p-2">
                                { props.action_elements.clone() }
                            </div>
                        )
                    }
                }

            </section>
        }
    }
}
