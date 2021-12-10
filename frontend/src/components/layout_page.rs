use yew::classes;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutPageProps {
    pub children: Children,
}

#[function_component(LayoutPage)]
pub fn layout_page(props: &LayoutPageProps) -> Html {
    html! {
        <div class={classes!("container", "mx-auto",  "relative", "p-4")}>
            <div class={classes!("prose", "mx-auto", "min-w-full")}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
