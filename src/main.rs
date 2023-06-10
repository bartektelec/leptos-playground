use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let dbl = move || count() * 2;

    let increment = move |_| {
        set_count(count() + 1);
    };

    view! { cx,
    <button on:click={increment}>"Click me: " {count} "and" {dbl}</button>
        <progress max="50" value=count />
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}

