extern crate yew_form;
#[macro_use]
extern crate yew_form_derive;

use std::rc::Rc;

use bounce::query::use_query_value;
use bounce::BounceRoot;
use yew::prelude::*;

// modules in this crate
mod api;
mod components;
mod store;
mod util;

use api::queries::plants::{query_plants, BounceQueryPlants};

use components::{
    icon::IconPlusSolid,
    layout_page::LayoutPage,
    modal::{Modal, ModalButton},
    plant_card::{PlantCard, PlantContainer},
    plant_new::PlantForm,
};
use util::mock_data;

#[function_component(LoadPlants)]
pub fn load_plants() -> Html {
    let variables = query_plants::Variables {};
    let plants = use_query_value::<BounceQueryPlants>(Rc::new(variables));
    html! {}
}

#[function_component(App)]
pub fn app() -> Html {
    /*
     * OLD LOADING MOCK DATA
     */
    // Because this is a ref, we need to dereference then clone to use the value
    // E.g. (*mock_data).clone()
    let mock_data = use_ref(|| mock_data::make_plant_previews(12));
    let plant_cards: Vec<Html> = (*mock_data)
        .iter()
        .map(|plant| {
            html! {
                <PlantCard
                    name={plant.name.clone()}
                    image={plant.image.clone()}
                    room={plant.room.clone()}
                    water_frequency={plant.water_frequency}
                    water_instructions={plant.water_instructions.clone()}
                    last_watered_date={plant.last_watered_date}
                    last_watered_by={plant.last_watered_by.clone()}
                />
            }
        })
        .collect();
    html! {
        <BounceRoot>
            <div id="portal-target" class={classes!("prose")} />
            <Modal>
                <PlantForm />
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
                  // Rooms
                <h2>{"Living Room - Attempting to Live Load GQL"}</h2>
                  <PlantContainer>
                    <LoadPlants />
                  </PlantContainer>
                <h2>{"Kitchen"}</h2>
                  <PlantContainer>
                    { plant_cards }
                  </PlantContainer>

                <h2>{"Bedroom"}</h2>
                <h2>{"Office"}</h2>
            </LayoutPage>
        </BounceRoot>
    }
}

pub fn main() {
    yew::start_app::<App>();
}
