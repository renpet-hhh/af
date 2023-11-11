use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PresetsProps {
    pub onselect: Callback<&'static str>,
}

static PRESETS: [&'static str; 4] = [
    /* PRESET 0 */
    "arg(a).
arg(b).
arg(c).
att(a,b).
att(b,c).
",
    /* PRESET 1 */
    "arg(a).
arg(b).
arg(c).
arg(d).
att(a,b).
att(b,a).
att(c,d).
att(d,c).",
    /* PRESET 2 */
    "arg(A).
arg(B).
arg(C).
arg(D).
arg(E).
arg(F).
arg(G).
arg(H).
att(B,H).
att(C,A).
att(G,A).
att(D,E).
att(H,D).
att(E,B).
att(C,F).",
    /* PRESET 3 */
    "arg(A0).
arg(A1).
arg(A2).
arg(A3).
arg(A4).
arg(A5).
arg(A6).
arg(A7).
arg(A8).
arg(A9).
arg(A10).
arg(A11).
arg(A12).
arg(A13).
arg(A14).
arg(A15).
arg(A16).
arg(A17).
arg(A18).
arg(A19).
arg(A20).
arg(A21).
arg(A22).
arg(A23).
arg(A24).
arg(A25).
arg(A26).
arg(A27).
arg(A28).
arg(A29).
att(A4,A18).
att(A8,A7).
att(A11,A21).
att(A4,A15).
att(A6,A15).
att(A17,A9).
att(A12,A3).
att(A2,A17).
att(A21,A27).
att(A16,A13).
att(A6,A0).
att(A25,A25).
att(A26,A12).
att(A6,A22).
att(A14,A14).
att(A23,A17).
att(A28,A4).
att(A28,A25).
att(A17,A15).
att(A12,A6).
att(A10,A6).
att(A20,A23).
att(A23,A2).
att(A23,A13).
att(A8,A21).
att(A1,A17).
att(A20,A12).
att(A4,A12).
att(A15,A2).
att(A11,A2).
att(A9,A20).
att(A28,A13).
att(A18,A23).
att(A26,A18).
att(A13,A24).
att(A26,A0).
att(A19,A9).
att(A4,A25).
att(A18,A14).
att(A6,A9).
att(A15,A25).
att(A13,A6).
att(A6,A14).
att(A19,A25).
att(A1,A14).
",
];

#[function_component]
pub fn Presets(props: &PresetsProps) -> Html {
    html! {
        <>
            <p class={classes!("ml-2")}>{ "Presets:" }</p>
            {
                (0..PRESETS.len()).map(|i| {
                    let handle_click = {
                        let onselect = props.onselect.clone();
                        Callback::from(move |_| {
                            onselect.emit(PRESETS[i]);
                        })
                    };
                    html! {
                        <button key={i} onclick={handle_click}
                            class={
                                classes!("m-1", "p-2", "bg-teal-600", "text-stone-100", "rounded")
                            }>{ i }</button>
                    }
                }).collect::<Html>()
            }
        </>
    }
}
