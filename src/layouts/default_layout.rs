use leptos::prelude::*;

#[component]
pub fn DefaultLayout(children: Children) -> impl IntoView {
    view! {
    <div class="flex flex-col h-screen">
        // Header
        <header class="bg-blue-500 text-white p-4 flex justify-between">
            <span>"My App Header"</span>
            <nav>
                <a href="/" class="mr-4">"Home"</a>
                <a href="/blog">"Blog"</a>
            </nav>
        </header>

        // Main content
        <main class="flex-1 overflow-y-auto p-4">
            {children()}
        </main>

        // Footer
        <footer class="bg-gray-200 text-center p-2 mt-auto">
            "Footer Content"
        </footer>
    </div>
    }
}
