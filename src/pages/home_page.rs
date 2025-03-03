use crate::components::demo::Demo;
use crate::store::counter::use_counter_store;
use leptos::html::Canvas;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn HomePage() -> impl IntoView {
    let on_press = move |_| {
        if let Some(window) = web_sys::window() {
            let _ = window.alert_with_message("Hello from Rust!");
        }
    };
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas: HtmlCanvasElement = canvas.into();
            if let Ok(Some(context)) = canvas.get_context("2d") {
                if let Ok(ctx) = context.dyn_into::<CanvasRenderingContext2d>() {
                    ctx.set_fill_style_str("blue");
                    ctx.fill_rect(10.0, 10.0, 100.0, 100.0);
                }
            }
        }
    });

    let store = use_counter_store();
    let store_for_demo = store.clone();
    let handle_demo_click = Callback::new(move |_| {
        store_for_demo.increment();
    });

    view! {
    <div class="space-y-6 p-6">
        {move || {
            let count = store.count.get();
            if count <= 2 || count> 6 {
                Some(view! {
                <h1 class="text-2xl font-bold text-gray-800">"Welcome to Leptos!"</h1>
                })
            } else {
                None
                }
            }
        }
        <div class="flex items-center space-x-4">
            <span class="text-lg">"Count:"</span>
            <span class="text-xl font-semibold text-blue-600">{move || store.count.get()}</span>
        </div>
        <canvas node_ref=canvas_ref class="border border-gray-300 rounded-md" />
        <button on:click=on_press
            class="px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 transition">
            "Alert"
        </button>
        <Demo text="Click (emit -> increment)" .to_string() on_demo_click=handle_demo_click />
    </div>
    }
}
