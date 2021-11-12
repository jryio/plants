use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::layout_page::LayoutPage;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
}

#[derive(Debug)]
pub struct Model;

impl Component for Model {
    // enum Msg  {
    //    Increment
    // }
    // type Message = Msg;
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    // Called when properties passed to the component change
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Link gives us access to this component's scope. We can dispatch messages so it can
        // update
        // let link = ctx.link();
        html! {
            <LayoutPage>
                    <h1>{"Plants"}</h1>
            </LayoutPage>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
