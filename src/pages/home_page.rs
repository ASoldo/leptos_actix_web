use crate::components::demo::Demo;
use leptos::html::Canvas;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
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
    let handle_demo_click = Callback::new(move |_| {
        *count.write() += 2;
        log::info!("Demo clicked! Count is now: {}", count.get());
    });

    view! {
    <h1 class="bg-amber-500 p-2 m-3 border-2 border-red-500">"Welcome to Leptos!"</h1>
    <button class="p-4 m-2 bg-blue-500 text-white rounded-3xl" on:click=on_click>"Click Me: " {count}</button>
    <canvas node_ref=canvas_ref class="border-black border-2" />
    <button on:click=on_press class="p-4 m-2 bg-green-500 rounded-3xl outline-red-500">"Alert"</button>
    <Demo text="Emit by clicking here and increment counter by 2" .to_string() on_demo_click=handle_demo_click />
    }
}
