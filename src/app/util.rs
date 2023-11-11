use web_sys::File;
use yew::{classes, Classes};

pub async fn read_file(file: File) -> Option<String> {
    let content = wasm_bindgen_futures::JsFuture::from(file.text()).await;
    match content {
        Err(_) => None,
        Ok(content) => {
            content.as_string()
        }
    }
}

pub fn flex_row() -> Classes {
    classes!("flex", "flex-row", "gap-1", "justify-center", "items-center")
}

pub fn flex_col() -> Classes {
    classes!("flex", "flex-col", "gap-1", "justify-center", "items-center")
}