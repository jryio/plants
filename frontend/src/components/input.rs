use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct LabelProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub no_margin: bool,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let margin = if props.no_margin { "" } else { "mt-4" };
    html! {
        <label class={classes!("block", margin )}>
            <span class={classes!("text-gray-700")}>
                {&props.label}
            </span>
            { props.children.clone() }
        </label>
    }
}
