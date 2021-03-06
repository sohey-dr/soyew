mod components;

use yew::prelude::*;
use components::header::Header;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Header />
    }
}

fn main() {
    yew::start_app::<App>();
}
