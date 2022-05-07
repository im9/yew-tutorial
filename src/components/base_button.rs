use yew::{function_component, html, Callback};

#[function_component(BaseButton)]
pub fn base_button() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        log::info!("click: {:?}", greeting);
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}
