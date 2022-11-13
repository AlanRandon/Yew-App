use yew::prelude::*;

use crate::hamburger::{self, Hamburger};

struct Link<'a> {
    text: &'a str,
    url: &'a str,
}

static LINKS: &[Link] = &[Link {
    text: "Home",
    url: "/",
}];

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
        let links = LINKS
            .iter()
            .map(|Link { text, url }| {
                html! {
                    <li>
                        <a href={ *url }>
                            { text }
                        </a>
                    </li>
                }
            })
            .collect::<Html>();
        html! {
            <header class={classes!(
                if self.links_visible {
                    "shadow-[0_0_0_100vmax_rgba(0_0_0_/_0.35)]"
                } else {
                    "shadow"
                },
                "sticky",
                "top-0",
                "z-30",
                "navbar",
                "bg-base-200",
                "w-full",
                "flex-wrap",
                "h-fit",
                "relative"
            )}>
                <div class="navbar-start gap-4 h-fit z-10">
                    <button onclick={link.callback(|_| Message::ToggleLinksVisible)} aria-haspopup="menu" aria-label="open navbar links" class="btn btn-ghost grid place-items-center">
                        <Hamburger btn_shape={if self.links_visible {
                            hamburger::BtnShape::Close
                        } else {
                            hamburger::BtnShape::Hamburger
                        }}/>
                    </button>
                    <h2 class="text-2xl navbar-center">{ "Generic App" }</h2>
                </div>
                <div class="[flex-basis:100%]" />
                <nav class={classes!(
                        if self.links_visible {
                            vec!["left-0", "opacity-100"]
                        } else {
                            vec!["scale-y-0", "opacity-0", "pointer-events-none"]
                        },
                        "menu",
                        "absolute",
                        "-z-50",
                        "transition-[transform,opacity]",
                        "bg-base-200",
                        "w-full",
                        "p-4",
                        "top-[100%]",
                        "left-0",
                        "origin-top",
                    )}>
                    { links }
                </nav>
            </header>
        }
    }
}
