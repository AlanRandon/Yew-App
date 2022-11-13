#![warn(clippy::pedantic)]

use card::Card;
use counter::Counter;
use lazy_static::lazy_static;
use navbar::Navbar;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod card;
mod counter;
mod hamburger;
mod navbar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math, js_name = random)]
    fn js_random() -> f32;
}

lazy_static! {
    static ref WORDS: Vec<&'static str> = include_str!("lorem.txt").split('\n').collect();
    static ref RNG: Mutex<SmallRng> = {
        let seed = js_random()
            .to_be_bytes()
            .into_iter()
            .fold(0_u64, |a, b| a + u64::from(b));
        Mutex::new(SmallRng::seed_from_u64(seed))
    };
}

fn lorem(count: u32) -> String {
    let mut text = String::new();
    let mut rng = RNG.lock().unwrap();
    for _ in 0..count {
        text += WORDS.choose(&mut *rng).expect("Failed to get word");
        text += " ";
    }
    text
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Navbar />
                <div class="hero min-h-fit">
                    <div class="hero-content text-center py-24 grid place-items-center gap-4">
                        <h1 class="text-5xl font-bold">{ "Meet Generic App" }</h1>
                        // Arrow SVG icon
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-16 h-16">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                        </svg>
                    </div>
                </div>
                <main class="flex flex-col items-center gap-4 p-4 w-full">
                    <div class="card-container grow gap-4">
                        {
                            for (0..3).map(|i| {
                                let image = format!("https://picsum.photos/536/354.webp?random={}", i);
                                html! {
                                    <Card
                                        title="Lorem Ipsum"
                                        image={image}
                                        action_elements={
                                            {
                                                html! {
                                                    { for
                                                        (0..{
                                                            let result = RNG.lock().unwrap().gen_range(0..5);
                                                            result
                                                        }).map(|_| {
                                                            html! {
                                                                <div class="badge badge-primary">{ lorem(1) }</div>
                                                            }
                                                        }).collect::<Vec<_>>()
                                                    }
                                                }
                                            }
                                        }
                                    >
                                        { lorem({
                                            let result = RNG.lock().unwrap().gen_range(5..40);
                                            result
                                        }) }
                                    </Card>
                                }
                            })
                        }
                    </div>
                    <Counter />
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
