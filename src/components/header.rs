use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <nav>
        <div>
          <a href="#">{ "Brand Logo" }</a>
        </div>
      </nav>
    }
}
