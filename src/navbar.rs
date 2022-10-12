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
            <header class="bg-base-200 p-4 flex gap-4 items-end shadow">
                <h2 class="text-2xl">{ "My App" }</h2>
                <nav class="flex gap-4 items-end justify-end block grow flex-wrap">
                {
                    LINKS.iter().map(|Link { text, url }| {
                        html!{
                            <a href={ *url } class="text-black focus-active-or-hover:text-primary-700 border border-0 border-transparent border-b-2 focus-or-active:border-current transition-colors">
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
