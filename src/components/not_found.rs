use leptos::*;
use leptos_router::Form;

/// 404 Not Found Page
/// #[allow(non_snake_case)]
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="main-content hero bg-base-200 rounded-lg">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-4xl font-semibold font-urbanis">ERROR: 404</h1>
                    <p class="py-6 font-light font-urbanist">This page is under construction</p>
                    <Form action="/" method="get">
                        <button class="btn btn-sm btn-success">Back to Home</button>
                    </Form>
                </div>
            </div>
        </div>
    }
}
