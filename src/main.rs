use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    layout_page::LayoutPage,
    plant_preview::{PlantContainer, PlantPreview, PlantPreviewProps},
    // plant_preview::PlantContainer,
    // plant_preview::PlantPreviewProps
};

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
        let living_room_plants = vec![
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Ladder Ivy"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
            PlantPreviewProps {
                name: String::from("Fiddle Leaf Fig"),
                image: String::from(""),
                room: String::from(""),
                water_frequency: 5,
                water_instructions: String::from(""),
                last_watered_date: String::from(""),
                last_watered_by: String::from(""),
            },
        ];
        html! {
            <LayoutPage>
                    <h1>{"Plants"}</h1>
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
    yew::start_app::<Model>();
}
