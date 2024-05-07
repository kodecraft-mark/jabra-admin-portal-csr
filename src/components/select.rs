use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq)]
pub struct Checkbox {
    pub checkbox_state: bool,
    pub item: String,
    pub value: String,
}

impl Checkbox {
    pub fn new(checkbox_state: bool, item: String, value:String) -> Self {
        Checkbox {
            checkbox_state,
            item,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq)]
pub struct Checkboxes {
    pub checkboxes: Vec<Checkbox>,
}

impl  Checkboxes{
    pub fn find_mut(&mut self, value: &str) -> Option<&mut Checkbox> {
        self.checkboxes.iter_mut().find(|i| { i.value.to_uppercase() == value.to_uppercase()})
    }
}

#[allow(non_snake_case)]
#[component]
pub fn SelectWithCheckbox(items: RwSignal<Checkboxes>, placeholder:RwSignal<String>) -> impl IntoView {

    let on_change = move |checkbox: Checkbox| {
        items.update(|u| {
            //check
        if let Some(existing_checkbox) = u
            .checkboxes
            .iter_mut()
            .find(|i| i.value.to_uppercase() == checkbox.value.to_uppercase())
        {
            existing_checkbox.checkbox_state = checkbox.checkbox_state;
        } else {
            u.checkboxes.push(checkbox);
        }
            
        });
    };

    let item_signal = RwSignal::new(String::from(""));
    let value_signal = RwSignal::new(String::from(""));

    let check = move || {
       
        let checkbox_state = if let Some(checkbox) = items.get().find_mut(value_signal.get().as_str()) {
            // Modify the checkbox if it exists
            if checkbox.checkbox_state {
                false
            } else {
                true
            }
        }else{
            false
        };
         on_change(Checkbox::new(checkbox_state, item_signal.get(), value_signal.get() ));
    };

    view! {
        <div tabindex="0" class="w-full dropdown">
        <div tabindex="0" role="button" class="block h-auto px-2 py-1 text-sm border-gray-800 rounded shadow-sm select select-sm text-success hover:shadow-sm hover:shadow-success bg-base-100">
            {
                move || {
                    let selected_items: Vec<Checkbox> = items.get().checkboxes.iter().filter(|i| i.checkbox_state).cloned().collect();
                    if selected_items.is_empty() {
                        view! {
                            <div>{placeholder.get()}</div>
                        }.into_view()
                    } else {
                        selected_items.into_iter().map(|item| {
                            view!{
                                <div class="mr-1 join border-success mb-[2px]">
                                    <button class="pointer-events-none btn join-item btn-xs border-success"><span>{item.item.clone()}</span></button>
                                    <button class="btn join-item btn-xs border-success hover:bg-success" prop:value={item.value.clone()}
                                        on:click = move |e| {
                                            let val = event_target_value(&e);
                                            value_signal.set(val.clone());
                                            check();    
                                             // If the clicked item is "ALL", unselect all other items and select "ALL"
                                            if val.clone() == "ALL" {
                                                items.update(|u| {
                                                    for item in &mut u.checkboxes {
                                                        if item.value == "ALL" {
                                                            item.checkbox_state = true;
                                                        } else {
                                                            item.checkbox_state = false;
                                                        }
                                                    }
                                                });
                                            }

                                            // If the clicked item is not "ALL" and "ALL" is currently selected, unselect "ALL"
                                            if val.clone() != "ALL" && items.get().checkboxes.iter().any(|i| i.value == "ALL" && i.checkbox_state) {
                                                items.update(|u| {
                                                    if let Some(all_item) = u.checkboxes.iter_mut().find(|i| i.value == "ALL") {
                                                        all_item.checkbox_state = false;
                                                    }
                                                });
                                            }

                                            // If no items are selected and "ALL" is present in the checkboxes data, select "ALL"
                                            if !items.get().checkboxes.iter().any(|i| i.checkbox_state) && items.get().checkboxes.iter().any(|i| i.value == "ALL") {
                                                items.update(|u| {
                                                    if let Some(all_item) = u.checkboxes.iter_mut().find(|i| i.value == "ALL") {
                                                        all_item.checkbox_state = true;
                                                    }
                                                });
                                            }
                                                                    
                                    }>x</button>
                                </div>

                            }
                        }).collect_view()
                    }
                }
                
            }
        </div>
        <ul tabindex="0" class="p-2 shadow menu dropdown-content z-[1] bg-base-300 border-success w-full max-h-60 overflow-y-auto !important block !important">
          {
            move || {
                items.get().checkboxes.into_iter().map(|item| {
                    let item_clone = item.clone();
                    view! {
                        <button class="w-full cursor-pointer"  prop:value={item.clone().value} on:click = move |e| {
                            let val = event_target_value(&e);
                            value_signal.set(val.clone());
                            check();    
                           
                            if val.clone() == "ALL" {
                                items.update(|u| {
                                    for item in &mut u.checkboxes {
                                        if item.value == "ALL" {
                                            item.checkbox_state = true;
                                        } else {
                                            item.checkbox_state = false;
                                        }
                                    }
                                });
                            }

                            // If the clicked item is not "ALL" and "ALL" is currently selected, unselect "ALL"
                            if val.clone() != "ALL" && items.get().checkboxes.iter().any(|i| i.value == "ALL" && i.checkbox_state) {
                                items.update(|u| {
                                    if let Some(all_item) = u.checkboxes.iter_mut().find(|i| i.value == "ALL") {
                                        all_item.checkbox_state = false;
                                    }
                                });
                            }

                            // If no items are selected and "ALL" is present in the checkboxes data, select "ALL"
                            if !items.get().checkboxes.iter().any(|i| i.checkbox_state) && items.get().checkboxes.iter().any(|i| i.value == "ALL") {
                                items.update(|u| {
                                    if let Some(all_item) = u.checkboxes.iter_mut().find(|i| i.value == "ALL") {
                                        all_item.checkbox_state = true;
                                    }
                                });
                            }
                        }>
                        <li>
                            <div class="form-control" prop:value={item.clone().value}>
                                
                                 <button class="label-text" prop:value={item.clone().value} 
                                >
                                <input type="checkbox" class="mr-2 rounded-none checkbox checkbox-success checkbox-xs" prop:value={item_clone.value} prop:checked={item_clone.checkbox_state}/>
                                    {item.item}</button>
                                
                            </div>
                        </li>
                        </button>
                    }
                }).collect_view()
            }            
          }
            
        </ul>
        </div>
    }
}