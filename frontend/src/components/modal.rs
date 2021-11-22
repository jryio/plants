use gloo_utils::document;
use log::info;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlDivElement, SvgElement};
use yew::events::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

use super::icon::IconXCircle;
use crate::store::{Action, Store};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ModalProps {
    // NOTE: It is super important that we prefix this prop with prop_or_default macro, this allows
    // us to have dispatch injected by yewdux rather than passing it in manually
    #[prop_or_default]
    pub dispatch: DispatchProps<ReducerStore<Store>>,
    #[prop_or_default]
    pub children: Children,
}

impl WithDispatchProps for ModalProps {
    type Store = ReducerStore<Store>;

    fn dispatch(&self) -> &DispatchProps<Self::Store> {
        &self.dispatch
    }
}

pub struct ModalBase;
pub type Modal = WithDispatch<ModalBase>;
impl Component for ModalBase {
    type Message = ();
    type Properties = ModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let modal_host = document()
            .get_element_by_id("portal-target")
            .expect("Missing element with ID #portal-target");

        let is_modal_open = ctx.props().dispatch.state().is_modal_open;

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

        // We want to write one function that can handle closing the modal whether you click on the
        // background or if you click on the SVG to close
        let on_close_modal = ctx.props().dispatch.callback(|e: MouseEvent| {
            let t1: Option<EventTarget> = e.target();
            let t2 = t1.clone();
            let div = t1.and_then(|t| t.dyn_into::<HtmlDivElement>().ok());
            let svg = t2.and_then(|t| t.dyn_into::<SvgElement>().ok());
            let div_id = div.map(|d| d.id()).unwrap_or_else(|| "".to_string());
            let svg_id = svg.map(|s| s.id()).unwrap_or_else(|| "".to_string());
            if div_id == "modal-background" || div_id == "close-icon" || svg_id == "close-icon" {
                Action::ModalClose
            } else {
                Action::NoOp
            }
        });

        let maybe_modal_children = if is_modal_open {
            html! {
                <>
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
                        { ctx.props().children.clone()}
                    </div>
                </div>
                </>
            }
        } else {
            html! {}
        };
        create_portal(maybe_modal_children, modal_host)
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ModalButtonProps {
    // NOTE: It is super important that we prefix this prop with prop_or_default macro, this allows
    // us to have dispatch injected by yewdux rather than passing it in manually
    #[prop_or_default]
    pub dispatch: DispatchProps<ReducerStore<Store>>,
    #[prop_or_default]
    pub children: Children,
}

impl WithDispatchProps for ModalButtonProps {
    type Store = ReducerStore<Store>;

    fn dispatch(&self) -> &DispatchProps<Self::Store> {
        &self.dispatch
    }
}

pub struct ModalButtonBase;
pub type ModalButton = WithDispatch<ModalButtonBase>;
impl Component for ModalButtonBase {
    type Message = ();
    type Properties = ModalButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_open_modal = ctx.props().dispatch.callback(|_| Action::ModalOpen);
        html! {
            <div onclick={on_open_modal}>
                { ctx.props().children.clone()}
            </div>
        }
    }
}
