use yew::prelude::*;

struct Link<'a> {
    text: &'a str,
    url: &'a str,
}

static LINKS: &[Link] = &[
    Link {
        text: "Home",
        url: "#1",
    },
    Link {
        text: "About",
        url: "#2",
    },
];

pub struct Navbar;

impl Component for Navbar {
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
            <header class="bg-base-200 p-4 mb-4 flex flex-row gap-4 items-end">
                <h1 class="text-2xl">{ "My App" }</h1>
                <nav class="flex flex-row gap-4 items-end justify-end block grow">
                {
                    LINKS.iter().map(|Link { text, url }| {
                        html!{
                            <a href={ *url } class="text-primary-500 hover:text-primary-400 focus-or-active:text-primary-600 border border-0 border-b-2 focus-active-or-hover:border-current transition-colors">
                                { text }
                            </a>
                        }
                    }).collect::<Html>()
                }
                </nav>
            </header>
        }
    }
}
