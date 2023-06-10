use leptos::*;

use crate::Store;
use crate::TodoItem;

#[component]
pub fn Todo(cx: Scope, todo: TodoItem) -> impl IntoView {
    let store = use_context::<Store>(cx).expect("to have found the store");

    let delete = move |_| {
        store.remove_todo.set(Some(todo.id));
    };

    view! {
        cx,
        <div>
            <div style="color: blue;">
            "#"{todo.id}": " {todo.title}
        <button on:click=delete>"DEL"</button>
            </div>
        </div>
    }
}
