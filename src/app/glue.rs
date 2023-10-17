use wasm_bindgen::prelude::*;

use js_sys::Array;

use super::af::Attack;

/* Transform Vec to JsValue */
fn str_array_js(v: Vec<String>) -> JsValue {
    JsValue::from(v.iter().map(|x| JsValue::from(x)).collect::<Array>())
}
fn usize_array_js(v: Vec<usize>) -> JsValue {
    JsValue::from(
        v.iter()
            .map(|&x| JsValue::from_f64(x as f64))
            .collect::<Array>(),
    )
}
/* ------------------------ */
/* Bindings */

#[wasm_bindgen]
extern "C" {
    fn updateVisNetwork(
        containerId: JsValue,
        labels: JsValue,
        attack_origin: JsValue,
        attack_target: JsValue,
        colors: JsValue,
    );
}

pub fn update_vis_network(
    container_id: &str,
    labels: Vec<String>,
    attacks: &Vec<Attack>,
    colors: Vec<String>,
) {
    let c = JsValue::from_str(container_id);
    let l = str_array_js(labels);
    let mut attack_origin = vec![];
    let mut attack_target = vec![];
    for Attack(origin, target) in attacks {
        attack_origin.push(*origin);
        attack_target.push(*target);
    }
    let o = usize_array_js(attack_origin);
    let t = usize_array_js(attack_target);
    let colors = str_array_js(colors);
    updateVisNetwork(c, l, o, t, colors);
}
