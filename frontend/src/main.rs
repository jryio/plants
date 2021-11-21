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
                <div id="portal-target" />
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

// #[function_component(App)]
// fn app() -> Html {
//     let store = use_reducer(|| State {
//         is_modal_open: false,
//     });

//     let on_open_modal = {
//         let store = store.clone();
//         Callback::from(move |_| store.dispatch(Action::ModalOpen))
//     };

//     let on_close_modal = {
//         let store = store.clone();
//         Callback::from(move |_| store.dispatch(Action::ModalClose))
//     };

//     let living_room_plants = mock_data::make_plant_previews(12);
//     html! {
//         <div>
//             <ContextProvider<State> context={(*store).clone()}>
//                 <div id="portal-target" />
//                 <Modal
//                     is_open={store.is_modal_open}
//                     on_close={on_close_modal}
//                 >
//                     <PlantNew />
//                 </Modal>
//                 <LayoutPage>
//                     <div class={classes!("flex", "flex-row", "flex-nowrap")}>
//                         // Header
//                         <h1
//                           class={classes!("flex-grow","bg-clip-text","text-transparent","bg-gradient-to-r","from-green-400","to-blue-700")}
//                           style="margin-bottom: 0;"
//                         >
//                             {"Munchkie House Plants"}
//                         </h1>
//                         // Add Plant
//                         <div onclick={on_open_modal} class={classes!("flex", "flex-row", "flex-shrink", "self-center", "p-2","mr-12", "gap-1", "hover:cursor-pointer")}>
//                             <svg
//                                 class={classes!("h-6", "w-6", "self-center", )}
//                                 fill="none"
//                                 viewBox="0 0 24 24"
//                                 stroke="currentColor"
//                                 xmlns="http://www.w3.org/2000/svg"
//                             >
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
//                           </svg>
//                           <span>{"Add Plant"}</span>
//                         </div>
//                       </div>
//                       <h2>{"Living Room"}</h2>
//                           <PlantContainer>
//                           {
//                               for living_room_plants.iter().cloned().map(|plant| {
//                                   html! {
//                                       <PlantCard
//                                           name={plant.name}
//                                           image={plant.image}
//                                           room={plant.room}
//                                           water_frequency={plant.water_frequency}
//                                           water_instructions={plant.water_instructions}
//                                           last_watered_date={plant.last_watered_date}
//                                           last_watered_by={plant.last_watered_by}
//                                       />
//                                   }
//                               })
//                           }
//                           </PlantContainer>
//                   <h2>{"Kitchen"}</h2>
//                   <h2>{"Bedroom"}</h2>
//                   <h2>{"Office"}</h2>
//                 </LayoutPage>
//         </ContextProvider<State>>
//         </div>
//     }
// }

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
