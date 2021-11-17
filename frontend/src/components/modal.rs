use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub is_visible: bool,
    pub children: Children,
}

// TODO: Subscribe to a piece of state. When true -> render children. Provide a
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_host = gloo_utils::document()
        .get_element_by_id("portal-target")
        .expect("a #portal-target element");

    create_portal(
        if props.is_visible {
            html! {
                // Background (TODO: Add click handler to close the modal here)
                <div class={classes!("min-w-full", "min-h-full", "bg-gray-600", "bg-opacity-50")}>
                    // Modal Container
                    <div class={classes!("max-w-screen-sm", "mx-auto")}>
                        {for props.children.iter()}
                    </div>
                </div>
            }
        } else {
            html! {}
        },
        modal_host,
    )
}
