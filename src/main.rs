use leptos::*;

mod components;

use components::Input::TodoInput;
use components::Todo::Todo;

#[derive(Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub checked: bool,
    pub active: bool,
}

#[derive(Clone)]
struct Store {
    pub remove_todo: WriteSignal<Option<usize>>,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<TodoItem>>(cx, vec![]);
    let (remove_id, set_remove_id) = create_signal::<Option<usize>>(cx, None);

    let add_todo = move |text: String| {
        let new_id = if let Some(last_todo) = todos.get().last() {
            last_todo.id + 1
        } else {
            0
        };

        set_todos.update(|collection| {
            collection.push(TodoItem {
                id: new_id,
                title: text,
                active: true,
                checked: false,
            })
        })
    };

    create_effect(cx, move |_| {
        if let Some(id_to_remove) = remove_id() {
            set_todos.update(move |collection| {
                if let Some(found) = collection.iter().position(|t| t.id == id_to_remove) {
                    collection.remove(found);
                }
            });
        };
    });

    provide_context(
        cx,
        Store {
            remove_todo: set_remove_id,
        },
    );
    view! { cx,
    <TodoInput on_add=add_todo />
    <For each=todos
    key=|t| {t.id}
    view=|cx, todo| view!{cx, <Todo todo />} />
    // <button on:click={increment}>"Click me: " {count} "and" {dbl}</button>
    //     <progress max="50" value=count />
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}

