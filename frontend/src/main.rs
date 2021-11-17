use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

mod components;
mod util;

use components::{
    layout_page::LayoutPage,
    modal::Modal,
    plant_card::{PlantCard, PlantContainer},
    plant_new::PlantNew,
};
use util::mock_data;

// Routes
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
}

// State
#[derive(Default, Clone)]
struct State {
    is_modal_visible: bool,
}

#[function_component(App)]
fn app() -> Html {
    let store = use_store::<BasicStore<State>>();
    let is_modal_visible = store
        .state()
        .map(|s| s.is_modal_visible)
        .unwrap_or_default();

    let living_room_plants = mock_data::make_plant_previews(12);
    html! {
        <div>
            <div id="portal-target" />
            <Modal is_visible={is_modal_visible} >
                <PlantNew />
            </Modal>
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
                        <svg
                            class={classes!("h-6", "w-6")}
                            // onclick={on_open_modal}
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                      </svg>
                    </div>
                  </div>
                  <h2>{"Living Room"}</h2>
                      <PlantContainer>
                      {
                          for living_room_plants.iter().cloned().map(|plant| {
                              html! {
                                  <PlantCard
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
        </div>
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
