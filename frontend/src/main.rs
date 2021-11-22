// use log::info;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

mod components;
mod store;
mod util;

use components::{
    icon::IconPlusSolid,
    layout_page::LayoutPage,
    modal::{Modal, ModalButton},
    plant_card::{PlantCard, PlantCardProps, PlantContainer},
    plant_new::PlantNew,
};
use store::{Action, Store};
use util::mock_data;

// Routes
// TODO: Make opening a modal possible by giving it a route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
}

pub struct App {
    mock_data: Vec<PlantCardProps>,
}
// type App = WithDispatch<AppBase>;
// type AppDispatchProps = DispatchProps<ReducerStore<Store>>;
impl Component for App {
    type Message = ();
    // type Properties = AppDispatchProps;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            mock_data: mock_data::make_plant_previews(12),
        }
    }

    // If we get a message don't update
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="portal-target" class={classes!("prose")} />
                <Modal>
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
                        // Add Plant
                        <ModalButton>
                            <div class={classes!("flex", "flex-row", "flex-shrink", "self-center", "p-2", "gap-1", "hover:cursor-pointer", "hover:rounded-md", "hover:bg-gray-200")} >
                                <IconPlusSolid class={classes!("h-6", "w-6", "self-center", )} />
                              <span>{"Add Plant"}</span>
                            </div>
                        </ModalButton>
                      </div>
                      <h2>{"Living Room"}</h2>
                          <PlantContainer>
                          {
                              for self.mock_data.iter().cloned().map(|plant| {
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
            </>
        }
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
