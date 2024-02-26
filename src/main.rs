use leptos::*;

fn main() {
    mount_to_body(|| view! {<App />});
    console_error_panic_hook::set_once();
}

#[component]
fn App() -> impl IntoView {
    let (count, setCount) = create_signal(0);
    view! {
        <button on:click = move|_|{
            setCount.update(|n| *n+=3 )
        }>

        "Click me: "
        {count}
    </button>
    }
}
