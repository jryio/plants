use yew::prelude::*;

#[function_component(PlantNew)]
pub fn plant_new() -> Html {
    // - form
    // - inputs
    // - validate
    // - submit (take a callback or a message to disaptch on success)
    html! {
        <>
            <h1>{"Add a new plant"}</h1>
            <form>
            </form>
        </>
    }
}
