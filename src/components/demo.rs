use leptos::prelude::*;

#[component]
pub fn Demo(text: String, on_demo_click: Callback<()>) -> impl IntoView {
    let handle_click = move |_| {
        on_demo_click.run(());
    };

    view! {
    <div class="p-4 m-2 rounded outline-amber-500 outline-2 outline-solid" on:click=handle_click>
        <p>{text}</p>
    </div>
    }
}
