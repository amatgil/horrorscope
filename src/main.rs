use leptos::*;

const HORRORSCOPES: &str = include_str!("../horrorscopes.txt");

fn random_prediction() -> &'static str {
    use rand::prelude::*;
    let predictions = HORRORSCOPES.split("\n").collect::<Vec<_>>();

    let i = rand::thread_rng().gen_range(0..predictions.len());
    return predictions[i];
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal("");

        view! {
            <p >
            {move || x.get() }
            </p>
            <button
                on:click={move |_| {
                    set_x.set(random_prediction());
                }}
                style="position: absolute"
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", x)
            >
                "Click to Move"
            </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
