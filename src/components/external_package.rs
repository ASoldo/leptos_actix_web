use crate::bindings::dayjs::*;
use leptos::prelude::*;
use leptos_meta::Script;

#[component]
pub fn ExternalPackages() -> impl IntoView {
    let (date, set_date) = signal(String::new());

    let show_date = move |_| {
        let formatted_date = dayjs().format("YYYY-MM-DD");
        set_date.set(formatted_date);
    };

    view! {
        <Script src="https://unpkg.com/dayjs@1.11.13/dayjs.min.js" />

        <div>
            <button class="bg-blue-500 rounded text-white p-4 m-2"
                    on:click=show_date>
                "Show Current Date (via local wrapper)"
            </button>

            <p>"Current date: " {date}</p>
        </div>
    }
}
