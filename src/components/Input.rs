use leptos::*;

#[component]
pub fn TodoInput<F>(cx: Scope, on_add: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let (input_value, set_input_value) = create_signal(cx, String::new());

    let change_input = move |ev| {
        set_input_value(event_target_value(&ev));
    };

    let click_add = move |_| {
        on_add(input_value());
        set_input_value("".to_string());
    };

    view! {cx,
        <div>
            <input
                type="text"
                on:change=change_input
                prop:value=input_value
                />
                <button
                    type="button"
                    on:click=click_add
                >"Add"</button>
        </div>
    }
}
