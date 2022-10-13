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
mod navbar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math, js_name = random)]
    fn js_random() -> f32;
}

lazy_static! {
    static ref WORDS: Vec<&'static str> = include_str!("lorem.txt").split_whitespace().collect();
    static ref RNG: Mutex<SmallRng> = {
        let seed = js_random()
            .to_be_bytes()
            .into_iter()
            .fold(0_u64, |a, b| a + u64::from(b));
        Mutex::new(SmallRng::seed_from_u64(seed))
    };
}

fn lorem() -> String {
    let mut text = String::new();
    let mut rng = RNG.lock().unwrap();
    for _ in 0..rng.gen_range(5..40) {
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
                <div class="w-full p-4 grid place-items-center bg-gradient-to-r from-primary-400 to-primary-analogous-400">
                    <h1 class="text-4xl font-black text-white">{ "My App" }</h1>
                </div>
                <main class="flex flex-col items-center gap-4 p-4 w-full">
                    <div class="card-container grow gap-4">
                        {
                            for (0..=10).map(|i| {
                                let image = format!("https://picsum.photos/536/354.webp?random={}", i);
                                html! {
                                    <Card title="Lorem Ipsum" image={image}>
                                        { lorem() }
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
