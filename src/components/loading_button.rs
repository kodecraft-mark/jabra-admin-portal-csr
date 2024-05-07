use leptos::*;
#[allow(non_snake_case)]
#[component]
pub fn LoadingButton() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center p-4 min-h-90vh">
            // <label class="rounded btn btn-md btn-success btn-outline">
            <span class="opacity-50 loading loading-spinner loading-sm text-neutral"></span>
            <span class="font-light opacity-50">Please wait . . .</span>
        // </label>
        </div>
    }
}
