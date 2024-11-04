use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputFieldProps {
    label: &'static str,
    value: Signal<String>,
    is_valid: Signal<bool>,
    validator: fn(&str) -> bool,
}

#[component]
pub fn InputField(mut props: InputFieldProps) -> Element {
    rsx! {
        div { class: "flex flex-col",
            label { class: "text-gray-700 font-semibold", "{props.label}" }
            input {
                class: "p-2 rounded border border-gray-300 mt-1 focus:outline-none focus:border-blue-500 shadow-sm",
                r#type: "text",
                value: "{props.value}",
                oninput: move |e| {
                    props.value.set(e.value().clone());
                    props.is_valid.set((props.validator)(&e.value()));
                }
            }
            if !(props.is_valid)() {
                p { class: "text-red-500 text-sm", "Invalid {props.label} format." }
            }
        }
    }
}
