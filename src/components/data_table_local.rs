use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::{arrow_down::ArrowDown, arrow_up::ArrowUp, download_anchor::DownloadCsvAnchor};

#[allow(non_snake_case)]
#[component]
pub fn DataTable(
    headers: RwSignal<Vec<String>>,
    keys: RwSignal<Vec<String>>,
    data: Signal<Vec<Value>>,
    #[prop(optional)] key_to_display_in_mobile: RwSignal<String>,
    #[prop(optional)] color: RwSignal<Vec<Value>>,
    #[prop(optional)] currency: RwSignal<Vec<Value>>,
    row_slice: RwSignal<usize>,
    #[prop(optional)] hasdownload: bool,
    #[prop(optional)] hasfilter: bool,
    nodatamessage: RwSignal<String>,
    #[prop(optional)] file_content: RwSignal<String>,
    #[prop(optional)] file_name: RwSignal<String>,
    #[prop(optional)] number_keys: RwSignal<Vec<String>>,
    #[prop(optional)] haspageslenght: bool,
) -> impl IntoView {
    let entries_start_signal = RwSignal::new(1);
    let entries_end_signal = RwSignal::new(0);

    let selected_header_params = RwSignal::new(String::from(""));
    let sort_asc = RwSignal::new(true);
    let selected_page = RwSignal::new(1);

    let filter_text = RwSignal::new(String::from(""));
    let clean_filter_text = move || filter_text.get().trim().to_lowercase();
    let data_size = Signal::derive(move || data.get().len());
    let filtered_data_len = RwSignal::new(0);

    let pages_entries = RwSignal::new(vec![5,10,15,20,25,50,100]);

    let header_length = headers.get_untracked().len();

    let sorted_data = RwSignal::new(Vec::<Value>::new());
    let filtered_data = Signal::derive({
        move || {
            //Sort the data based on the selected header
            sorted_data.set(sort_table(
                data.get(),
                sort_asc.get(),
                selected_header_params.get(),
                number_keys.get(),
            ));

            // Filter the data based on the filter text
            let filtered_sorted_data = sorted_data
                .get()
                .iter()
                .filter(|row| match row {
                    serde_json::Value::Object(obj) => obj.values().any(|val| {
                        if let Some(s) = val.as_str() {
                            s.to_lowercase().contains(&clean_filter_text())
                        } else {
                            false
                        }
                    }),
                    serde_json::Value::Array(arr) => arr.iter().any(|val| {
                        if let Some(s) = val.as_str() {
                            s.to_lowercase().contains(&clean_filter_text())
                        } else {
                            false
                        }
                    }),
                    _ => false,
                })
                .cloned()
                .collect::<Vec<_>>();

            if filtered_sorted_data.len() < row_slice.get() {
                selected_page.set(1);
            }

            let start_index = (selected_page.get() - 1) * row_slice.get();
            let end_index = (start_index + row_slice.get()).min(filtered_sorted_data.len());
            filtered_data_len.set(filtered_sorted_data.len());
            if start_index > end_index {
                selected_page.set(1);
                entries_start_signal.set(0);
                entries_end_signal.set(row_slice.get());
            } else {
                entries_start_signal.set(start_index);
                entries_end_signal.set(end_index);
            }
            // entries_end_signal.set(end_index);
            let filtered_data_slice =
                filtered_sorted_data[entries_start_signal.get()..entries_end_signal.get()].to_vec();
            filtered_data_slice
        }
    });

    // let filtered_data_len = move || filtered_data.get().len();
    let page_slice_size = Signal::derive(move || {
        let filtered_len = filtered_data_len.get();
        if filtered_len % row_slice.get() != 0 {
            filtered_len / row_slice.get() + 1
        } else {
            filtered_len / row_slice.get()
        }
    });

    view! {
        <Show
            when = move || {data_size() > 0}
            fallback = move || view! {
                <div class = "p-5">
                    <span class = "opacity-50 font-extralight">{nodatamessage}</span>
                </div>
            } >
            <div class = "p-4 overflow-auto">
                <div class="flex justify-between w-full">
                    <div class="flex items-center justify-start gap-4 join">
                    { 
                        if haspageslenght {
                            view! {
                                <select class = "block w-full mr-1 text-xs border-gray-800 rounded shadow-md select-sm hover:shadow-sm hover:shadow-success bg-base-100" name="row_slice"
                                    on:change = move |e| {
                                        let val = event_target_value(&e).parse::<usize>().unwrap();
                                        row_slice.set(val);
                                    }
                                >

                                {
                                    move || {
                                        pages_entries.get().into_iter().map(|cp| {
                                            view! {
                                                <option prop:selected = row_slice.get() == cp value = cp.to_string()>{cp}</option>
                                            }
                                        }).collect_view()
                                    }
                                }
                            </select>  
                            }.into_view()
                        }else{
                            view! {

                            }.into_view()
                        } 
                    }

                        { if hasdownload {
                            view! {
                                <DownloadCsvAnchor content = file_content.get() file_name = file_name.get()/>
                            }.into_view()
                        }else{
                            view! {

                            }.into_view()
                        }
                    }
                    </div>
                    <div class="flex justify-end join">
                    { if hasfilter {
                        view! {<div style="margin-bottom: 0.375rem;">  <span class="mr-2 label-text">Search: </span>
                            <input
                                type="text"
                                class="input input-sm input-info focus:outline-none focus:shadow-outline "
                                placeholder=""
                                bind:value=filter_text
                                on:input=move |event| { // Fix: Added type annotation for event parameter
                                    filter_text.set(event_target_value(&event));
                                }
                            />
                        </div>}.into_view()
                        }else{view! {<div style="margin-bottom: 0.375rem;"></div>}.into_view()}
                    }
                    </div>
                </div>
                <div> //class= format!("overflow-auto {}",{pagesheight.get()})>
                <table class = "table table-xs table-zebra-zebra">
                <thead>
                    <tr class = "hidden text-sm uppercase text-success px924:contents">
                    {
                        headers.get().into_iter().enumerate().map(|(index, key_name)| {
                            // let selected_header_param = keys.get()[index];
                            view! {
                                <th class = "text-white bg-opacity-50 cursor-pointer bg-success" on:click = move |_| //class = "sticky top-0 z-20 cursor-pointer bg-base-100" on:click = move |_|
                                    {
                                    selected_header_params.set(keys.get()[index].clone());
                                    sort_asc.update(|s| *s = !*s)
                                }>
                                <div class = "flex justify-between">
                                    <span class = "flex-0">{key_name}</span>
                                    <span  class = "flex-0">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                        <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
                                        </svg>
                                    </span>
                                </div>
                            </th>}
                        }).collect_view()
                    }
                    </tr>
                </thead>
                <tbody>
                {
                    move || {
                        filtered_data().into_iter().map(|d| {
                            let hide_per_cell = RwSignal::new(true);
                            let key_to_display = key_to_display_in_mobile.get();

                            let header_display = d.get(&key_to_display)
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "Click to show more...".to_string()).replace('"', "");

                            view! {
                                <tr class="px924:hidden">
                                    <td colspan = format!("{}", header_length)>
                                    {
                                        view! {
                                            <button class = "flex justify-start w-full gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success" on:click = move |_| hide_per_cell.update(|c| *c = !*c) >
                                                <div class = "text-xs">
                                                    {&header_display}
                                                </div>
                                                <div class = "flex justify-end flex-1">
                                                    <Show when = move || hide_per_cell.get() fallback = move || view! {<ArrowDown />}>
                                                        <ArrowUp />
                                                    </Show>
                                                </div>
                                            </button>
                                        }.into_view()
                                    }
                                    </td>
                                </tr>

                                // MOBILE VIEW

                                <TableRowMobile _val = d.clone() dksource = keys.get() keys = color.get() currency = currency.get() mobile_view = true hide_per_cell headers />

                                // DESKTOP VIEW

                               // <div class="hidden px924:contents">
                                    <tr class = "uppercase hover:opacity-50">
                                    <div class="hidden px924:contents">
                                        <TableCell _val = d dksource = keys.get() keys = color.get() currency = currency.get() />
                                        </div>
                                    </tr>
                                //</div>
        
                            }
                        }).collect_view()
                    }

                }
                </tbody>
                <tfoot >//class="sticky bottom-0 z-20 bg-base-100">
                    <tr>
                        <td colspan = {headers.get().len()}>
                            <TablePagination size = page_slice_size selected_page = selected_page entries_start = entries_start_signal entries_end = entries_end_signal total_entries = filtered_data_len />
                        </td>
                    </tr>
                </tfoot>
                </table>
                </div>
                </div>
        </Show>
    }
}

pub fn sort_table(
    mut data_table: Vec<serde_json::Value>,
    sort_type: bool,
    sort_by_params: String,
    number_params: Vec<String>,
) -> Vec<serde_json::Value> {
    // Sort the vector if needed
    if number_params.contains(&sort_by_params) {
        if sort_type {
            data_table.sort_by(|a, b| {
                let val_a = a
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let val_b = b
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                val_a
                    .partial_cmp(&val_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            data_table.sort_by(|a, b| {
                let val_a = a
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let val_b = b
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                val_b
                    .partial_cmp(&val_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
    } else {
        if sort_type {
            data_table.sort_by(|a, b| {
                let name_a = a
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let name_b = b
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                name_a.cmp(name_b)
            });
        } else {
            data_table.sort_by(|a, b| {
                let name_a = a
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let name_b = b
                    .get(sort_by_params.to_lowercase())
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                name_b.cmp(name_a)
            });
        }
    }

    data_table
}

#[allow(non_snake_case)]
#[component]
pub fn TablePagination(
    size: Signal<usize>,
    selected_page: RwSignal<usize>,
    entries_start: RwSignal<usize>,
    entries_end: RwSignal<usize>,
    total_entries: RwSignal<usize>,
) -> impl IntoView {
    view! {
        <Show when = move || {size.get() > 1} >
        <div class="flex justify-between w-full">
            <div class="flex items-center justify-start join">
                <span>{format!("Showing {} to {} of {} entries", entries_start.get()+1, entries_end.get(), total_entries.get())}</span>
            </div>
            <div class="flex justify-end join">
                {
                    move || {
                        let num_pages = size.get();
                        let current_page = selected_page.get();
                        let pages_to_display: Vec<usize> = if num_pages <= 5 {
                            (1..=num_pages).collect()
                        } else {
                            let start_page = if current_page <= 3 { 1 } else if current_page > num_pages - 2 { num_pages - 4 } else { current_page - 2 };
                            (start_page..=start_page+4).collect()
                        };

                        let previous_button = if current_page > 1 {
                            view! {
                                <button class="btn btn-xs m-0.5" on:click=move |_| {
                                    selected_page.set(current_page - 1);
                                }>{"Previous"}</button>
                            }
                        } else {
                            view! {
                                <button class="btn btn-xs m-0.5" disabled >{"Previous"}</button>
                            }
                        };

                        let next_button = if current_page < num_pages {
                            view! {
                                <button class="btn btn-xs m-0.5" on:click=move |_| {
                                    selected_page.set(current_page + 1);
                                }>{"Next"}</button>
                            }
                        } else {
                            view! {
                                <button class="btn btn-xs m-0.5" disabled >{"Next"}</button>
                            }
                        };

                        let page_buttons = pages_to_display.into_iter().map(|i| {
                            view! {
                                <button class = {move || if current_page == i {"join-item btn btn-xs bg-base-content bg-opacity-10 m-0.5"} else {"join-item btn btn-xs m-0.5"}} on:click = move |_| {
                                    selected_page.set(i);
                                } >{i}</button>
                            }
                        }).collect_view();
                        view! {
                            <>
                                {previous_button}
                                {page_buttons}
                                {next_button}
                            </>
                        }
                    }
                }
            </div>
        </div>
        </Show>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn TableRowMobile(
    _val: Value,
    dksource: Vec<String>,
    keys: Vec<Value>,
    currency: Vec<Value>,
    mobile_view: bool,
    hide_per_cell: RwSignal<bool>,
    headers: RwSignal<Vec<String>>,
) -> impl IntoView {

    let header_length = headers.get_untracked().len();
    let slice_of_strings: &[String] = dksource.as_slice();
    slice_of_strings
        .into_iter()
        .enumerate()
        .map(|(index, _d)| {
            let header = headers.get_untracked().get(index).unwrap_or(&"".to_string()).clone();
            let binding = _val[_d].to_string();
            let stringval = match &_val[_d] {
                Value::String(s) => s,
                _ => &binding,
            };
            let mut currencytxt: String = "".to_string();
            if !currency.is_empty() {
                if stringval != "- -" {
                    let current_str = serde_json
                        ::to_string(&currency)
                        .expect("Failed to convert to JSON string");

                    let parsed_currency: Vec<CurrencyItem> = serde_json
                        ::from_str(&current_str)
                        .expect("Failed to parse JSON");
                    for item in &parsed_currency {
                        if item.key.contains(&_d.to_string()) {
                            match &_val[item.value.to_string()] {
                                Value::String(s) => {
                                    currencytxt = s.to_string();
                                }
                                _ => {
                                    currencytxt = "".to_string();
                                }
                            }
                        }
                    }
                }
            }
            let mut style: Option<String> = None;
            keys.iter().any(|key| {
                match key.get(_d.to_string()) {
                    Some(value) => {
                        style = loop_value(value, stringval);
                        true
                    }
                    None => {
                        style = None;
                        false
                    }
                }
            });


            view! {
                <tr prop:hidden = move || hide_per_cell.get() class = "uppercase hover:opacity-50">
                    <td colspan = format!("{}", header_length) class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">{header}: </span><span class="px924:hidden" class = {if stringval != "- -" {style.clone()}else{None}}>{stringval}</span><span class = "text-xs opacity-50 font-extralight px924:hidden">{format!(" {}",currencytxt)}</span></td>
                </tr>
            }

        })
        .collect_view()
}

#[allow(non_snake_case)]
#[component]
pub fn TableCell(
    _val: Value,
    dksource: Vec<String>,
    keys: Vec<Value>,
    currency: Vec<Value>,
) -> impl IntoView {
    let slice_of_strings: &[String] = dksource.as_slice();
    slice_of_strings
        .into_iter()
        .map(|_d| {
            let binding = _val[_d].to_string();
            let stringval = match &_val[_d] {
                Value::String(s) => s,
                _ => &binding,
            };
            let mut currencytxt: String = "".to_string();
            if !currency.is_empty() {
                if stringval != "- -" {
                    let current_str = serde_json
                        ::to_string(&currency)
                        .expect("Failed to convert to JSON string");

                    let parsed_currency: Vec<CurrencyItem> = serde_json
                        ::from_str(&current_str)
                        .expect("Failed to parse JSON");
                    for item in &parsed_currency {
                        if item.key.contains(&_d.to_string()) {
                            match &_val[item.value.to_string()] {
                                Value::String(s) => {
                                    currencytxt = s.to_string();
                                }
                                _ => {
                                    currencytxt = "".to_string();
                                }
                            }
                        }
                    }
                }
            }
            let mut style: Option<String> = None;
            keys.iter().any(|key| {
                match key.get(_d.to_string()) {
                    Some(value) => {
                        style = loop_value(value, stringval);
                        true
                    }
                    None => {
                        style = None;
                        false
                    }
                }
            });

            view! {
                <td><span class = {if stringval != "- -" {style}else{None}}>{stringval}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}",currencytxt)}</span></td>
            }
        })
        .collect_view()
}

pub fn loop_value(value: &Value, stringval: &String) -> Option<String> {
    match value {
        Value::Object(obj) => {
            if let Some(key) = obj.get("key").and_then(|v| v.as_str()) {
                if key.to_uppercase() == stringval.to_uppercase() {
                    let style = obj.get("style").and_then(|v| v.as_str()).unwrap_or("");
                    return Some(style.to_string());
                } else {
                    None
                }
            } else {
                None
            }
        }
        Value::Array(arr) => {
            for obj in arr.iter() {
                if let Some(key) = obj.get("key").and_then(|v| v.as_str()) {
                    if stringval.to_uppercase().contains(&key.to_uppercase()) {
                        let style = obj.get("style").and_then(|v| v.as_str()).unwrap_or("");
                        return Some(style.to_string());
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrencyItem {
    key: String,
    value: String,
}