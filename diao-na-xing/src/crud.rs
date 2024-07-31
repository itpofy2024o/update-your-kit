use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
}

pub fn CrudApp(cx: Scope) -> Element {
    let tasks = use_state(&cx, || {
        LocalStorage::get("tasks").unwrap_or_else(|_| HashMap::new())
    });

    let new_task_description = use_state(&cx, || String::new());

    let add_task = {
        let tasks = tasks.clone();
        let new_task_description = new_task_description.clone();
        move |_| {
            let id = tasks.len();
            tasks.with_mut(|tasks| {
                tasks.insert(id, Task {
                    id,
                    description: new_task_description.get().clone(),
                });
                LocalStorage::set("tasks", tasks).unwrap();
            });
            new_task_description.set(String::new());
        }
    };

    let delete_task = {
        let tasks = tasks.clone();
        move |id: usize| {
            tasks.with_mut(|tasks| {
                tasks.remove(&id);
                LocalStorage::set("tasks", tasks).unwrap();
            });
        }
    };

    cx.render(rsx!(
        div {
            h1 { "CRUD App" }
            input {
                value: "{new_task_description}",
                oninput: |e| new_task_description.set(e.value.clone()),
            }
            button { onclick: add_task, "Add Task" }
            ul {
                tasks.iter().map(|(id, task)| rsx!(
                    li {
                        key: "{id}",
                        "{task.description}"
                        button { onclick: move |_| delete_task(*id), "Delete" }
                    }
                ))
            }
        }
    ))
}
