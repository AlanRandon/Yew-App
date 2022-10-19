use yew::prelude::*;

use crate::hamburger::Hamburger;

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
    Link {
        text: "Really, Really, Really Verbose Name For A Page That Is A Bit Ridiculous",
        url: "#3",
    },
];

pub struct Navbar {
    links_visible: bool,
}

pub enum Message {
    ToggleLinksVisible,
}

impl Component for Navbar {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            links_visible: false,
        }
    }

    // Returns `true` if a re-render is needed
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleLinksVisible => {
                self.links_visible = !self.links_visible;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let links = LINKS.iter().map(|Link { text, url }| {
            html!{
                <a href={ *url } class="text-black focus-active-or-hover:text-primary-700 border border-0 border-transparent border-b-2 focus-or-active:border-current transition-colors shrink">
                    { text }
                </a>
            }
        }).collect::<Html>();
        html! {
            <header class="sticky z-10 top-0">
                <div class="flex gap-4 items-end justify-between block grow p-4 bg-base-200 shadow">
                    <h2 class="text-2xl">{ "My App" }</h2>
                    <Hamburger/>
                    <button onclick={link.callback(|_| Message::ToggleLinksVisible)}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                        </svg>
                    </button>
                </div>
                <nav class={classes!(
                    (!self.links_visible)
                        .then(|| vec!["top-0", "pointer-events-none", "opacity-0"])
                        .unwrap_or_else(|| vec!["top-[100%]", "opacity-100"]),
                        "transition-[top,opacity]",
                        "absolute",
                        "left-0",
                        "p-4",
                        "bg-base-200",
                        "z-[-1]",
                        "flex",
                        "flex-col",
                        "shadow-[0_0_0_100vmax_rgb(0_0_0_/_0.25)]",
                        "items-center"
                    )}>
                    { links }
                </nav>
            </header>
        }
    }
}
