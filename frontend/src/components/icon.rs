use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct IconProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconPlusSolid)]
pub fn icon_plus_solid(props: &IconProps) -> Html {
    html! {
        <svg
            onclick={props.onclick.clone()}
            id={props.id.clone()}
            class={classes!(props.class.clone())}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path id={props.id.clone()} stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
    }
}

#[function_component(IconXCircle)]
pub fn icon_x_circle(props: &IconProps) -> Html {
    html! {
        <svg
            onclick={props.onclick.clone()}
            id={props.id.clone()}
            class={classes!(props.class.clone())}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            xmlns="http://www.w3.org/2000/svg"
        >
          <path id={props.id.clone()} strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
    }
}
