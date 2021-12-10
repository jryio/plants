use bounce::{use_slice_dispatch, use_slice_value};
use gloo::console::debug;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlDivElement, HtmlHeadElement, SvgElement};
use yew::events::MouseEvent;
use yew::prelude::*;

use super::icon::IconXCircle;
use crate::store::{Action, Store};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_modal_open = use_slice_value::<Store>().is_modal_open;
    let modal_dispatch = use_slice_dispatch::<Store>();

    let body_portal = use_ref(|| {
        let document_head: HtmlHeadElement = gloo_utils::document()
            .head()
            .expect("head element to be present");
        let body_portal = create_portal(
            html! {
                <style>
                    {"body { overflow: hidden; }"}
                </style>
            },
            document_head.into(),
        );
        body_portal
    });

    let modal_host = document()
        .get_element_by_id("portal-target")
        .expect("Missing element with ID #portal-target");

    let id_modal_background = String::from("modal-background");
    let id_modal_window = String::from("modal-window");
    let id_modal_content = String::from("modal-content");
    let id_close_icon = String::from("close-icon");

    let classes_background = classes!(
        "absolute",
        "z-10",
        "min-w-full",
        "min-h-full",
        "bg-gray-600",
        "bg-opacity-80",
        "hover:cursor-pointer"
    );

    let classes_absolute_container = classes!(
        "absolute",
        "z-20",
        "container",
        "top-1/2",
        "left-1/2",
        "transform",
        "-translate-x-1/2",
        "-translate-y-1/2",
        "min-w-screen-sm",
        "max-w-screen-sm",
        "h-4/6",
        "p-4",
        "bg-white",
        "rounded-md",
    );

    // Render a close button here
    let classes_relative_container = classes!("relative", "h-full", "overflow-scroll");

    let classes_icon_close = classes!(
        "absolute",
        "top-0",
        "right-0",
        "h-6",
        "w-6",
        "self-center",
        "hover:bg-gray-200",
        "hover:rounded-full",
        "hover:cursor-pointer"
    );

    let on_close_modal = Callback::from(move |e: MouseEvent| {
        let t1: Option<EventTarget> = e.target();
        let t2 = t1.clone();
        let div = t1.and_then(|t| t.dyn_into::<HtmlDivElement>().ok());
        let svg = t2.and_then(|t| t.dyn_into::<SvgElement>().ok());
        let div_id = div.map(|d| d.id()).unwrap_or_else(|| "".to_string());
        let svg_id = svg.map(|s| s.id()).unwrap_or_else(|| "".to_string());
        if div_id == "modal-background" || div_id == "close-icon" || svg_id == "close-icon" {
            modal_dispatch(Action::ModalClose)
        } else {
            modal_dispatch(Action::None)
        }
    });

    let maybe_modal_children = if is_modal_open {
        html! {
            <>
                // Prevent the page from scrolling while the modal is open
                { (*body_portal).clone() }
                <div
                    onclick={&on_close_modal}
                    id={id_modal_background}
                    class={classes_background}
                >
                </div>

                // Modal
                <div id={id_modal_window} class={classes_absolute_container}>
                    // Content
                    <div id={id_modal_content} class={classes_relative_container}>
                        <IconXCircle
                            onclick={&on_close_modal}
                            id={id_close_icon}
                            class={classes_icon_close}
                        />
                        { for props.children.iter()}
                    </div>
                </div>
            </>
        }
    } else {
        html! {}
    };
    create_portal(maybe_modal_children, modal_host)
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ModalButtonProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html {
    let modal_dispatch = use_slice_dispatch::<Store>();
    let on_open_modal = {
        let modal_dispatch = modal_dispatch.clone();
        Callback::from(move |_: MouseEvent| modal_dispatch(Action::ModalOpen))
    };
    html! {
        <div onclick={on_open_modal}>
            { for props.children.iter()}
        </div>
    }
}
