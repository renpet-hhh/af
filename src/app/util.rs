use web_sys::File;

pub async fn read_file(file: File) -> Option<String> {
    let content = wasm_bindgen_futures::JsFuture::from(file.text()).await;
    match content {
        Err(err) => None,
        Ok(content) => {
            content.as_string()
        }
    }
}
