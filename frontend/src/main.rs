use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod lib;
use components::{
    layout_page::LayoutPage,
    plant_preview::{PlantContainer, PlantPreview},
};
use lib::mock_data;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
}

#[derive(Debug)]
pub struct Model;

impl Component for Model {
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
        let living_room_plants = mock_data::make_plant_previews(12);
        html! {
            <LayoutPage>
              <div class={classes!("flex", "flex-row", "flex-nowrap")}>
                // Header
                <h1
                  class={classes!("flex-grow","bg-clip-text","text-transparent","bg-gradient-to-r","from-green-400","to-blue-700")}
                  style="margin-bottom: 0;"
                >
                  {"Munchkie House Plants"}
                </h1>
                // Add Icon
                <div class={classes!("flex-shrink", "self-center", "p-2","mr-6")}>
                  <svg class={classes!("h-6", "w-6")} fill="none" viewBox="0 0 24 24" stroke="currentColor" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </div>
              </div>
              <h2>{"Living Room"}</h2>
                  <PlantContainer>
                  {
                      for living_room_plants.iter().cloned().map(|plant| {
                          html! {
                              <PlantPreview
                                  name={plant.name}
                                  image={plant.image}
                                  room={plant.room}
                                  water_frequency={plant.water_frequency}
                                  water_instructions={plant.water_instructions}
                                  last_watered_date={plant.last_watered_date}
                                  last_watered_by={plant.last_watered_by}
                              />
                          }
                      })
                  }
                  </PlantContainer>
              <h2>{"Kitchen"}</h2>
              <h2>{"Bedroom"}</h2>
              <h2>{"Office"}</h2>
            </LayoutPage>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
