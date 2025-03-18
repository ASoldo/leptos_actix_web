use js_sys::{Function, Reflect};
use leptos::prelude::*;
use leptos_meta::Script;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn ExternalPackages() -> impl IntoView {
    let (date, set_date) = signal(String::new());

    let show_date = move |_| {
        set_date.set("Loading...".to_string());

        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                let dayjs_fn = js_sys::Reflect::get(&window, &JsValue::from_str("dayjs"))
                    .unwrap()
                    .dyn_into::<Function>()
                    .unwrap();

                let now = dayjs_fn.call0(&JsValue::NULL).unwrap();

                let formatted_date = Reflect::get(&now, &JsValue::from_str("format"))
                    .unwrap()
                    .dyn_into::<Function>()
                    .unwrap()
                    .call0(&now)
                    .unwrap()
                    .as_string()
                    .unwrap_or_default();

                set_date.set(formatted_date);
            }
        });
    };

    view! {
        <Script src="https://unpkg.com/dayjs@1.11.13/dayjs.min.js"/>

        <div>
            <button
                class="bg-blue-500 rounded text-white p-4 m-2"
                on:click=show_date
            >
                "Show Current Date (via CDN)"
            </button>

            <p>"Current date: " {date}</p>
        </div>
    }
}
