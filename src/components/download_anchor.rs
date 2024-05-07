use leptos::*;
#[allow(non_snake_case)]
#[component]
pub fn DownloadCsvAnchor(
    content: String,
    file_name: String,
    #[prop(optional)] button_name: String,
) -> impl IntoView {
    use wasm_bindgen::JsValue;
    use web_sys::{
        js_sys::{Array, Uint8Array},
        Blob, BlobPropertyBag,
    };
    let new_file_name = move || {
        let utc = chrono::Utc::now();
        let utc_local = utc.with_timezone(&chrono::Local);
        let formatted_local = utc_local.format("%Y%m%d_%H%M%S").to_string();
        format!("{}_{}.csv", formatted_local, file_name)
    };
    let button_placeholder = move || match button_name.len() > 0 {
        true => button_name,
        false => String::from("Download"),
    };
    let download = move || {
        let uint8arr = Uint8Array::new(&unsafe { Uint8Array::view(&content.as_bytes()) }.into());
        let array = Array::new();
        array.push(&uint8arr.buffer());
        let file = Blob::new_with_u8_array_sequence_and_options(
            &JsValue::from(array),
            BlobPropertyBag::new().type_("text/csv"),
        )
        .unwrap();
        let doc = leptos_dom::document();
        let hyperlink = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlAnchorElement>(
            doc.create_element("a").unwrap(),
        )
        .unwrap();
        hyperlink.set_download(new_file_name().as_str());
        let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
        hyperlink.set_href(&url);
        hyperlink.click();
        hyperlink.remove();
    };
    view! {
        <div>
            <button
                class="font-normal btn btn-sm btn-ghost bg-base-100 rounded-sm"
                on:click=move |_| download()
            >
                <div class="flex gap-2 justify-normal text-center items-center content-center">
                    <span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class="w-4 h-4"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm4.75 6.75a.75.75 0 011.5 0v2.546l.943-1.048a.75.75 0 011.114 1.004l-2.25 2.5a.75.75 0 01-1.114 0l-2.25-2.5a.75.75 0 111.114-1.004l.943 1.048V8.75z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </span>
                    <span class="font-extralight">{button_placeholder()}</span>
                </div>
            </button>
        </div>
    }
}
