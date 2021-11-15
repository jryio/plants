use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlantContainerProps {
    pub children: Children,
}

#[function_component(PlantContainer)]
pub fn plant_container(props: &PlantContainerProps) -> Html {
    html! {
      <div class={classes!("flex", "flex-row", "flex-wrap", "gap-8" )}>
        { props.children.clone() }
      </div>
    }
}

// TODO: Compute status of plant based on last water date
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PlantPreviewProps {
    pub name: String,
    pub image: String,
    pub room: String,
    pub water_frequency: usize,
    pub water_instructions: String,
    pub last_watered_date: String,
    pub last_watered_by: String,
}

#[function_component(PlantPreview)]
pub fn plant_preview(props: &PlantPreviewProps) -> Html {
    println!("props = {:?}", props);
    html! {
      // Container
      <div class={classes!("flex", "flex-col","w-72", "h-72","bg-green-100")}>
        // Image
        <img
          src={String::from("https://hips.hearstapps.com/vader-prod.s3.amazonaws.com/1557177323-pilea-peperomioides-money-plant-in-the-pot-single-royalty-free-image-917778022-1557177295.jpg?crop=1.00xw:0.668xh;0,0.269xh&resize=480:*")}
          class={classes!("flex-shrink-0","h-4/6","object-cover","self-start", "w-full", "max-w-full", "bg-gray-400")}
        />
        <div class={classes!("p-2", "h-2/6")}>
          // Name
          <div class={classes!("flex-shrink-1", "flex-wrap", "overflow-hidden", "opactiy-100")}>
            <div class={classes!("text-lg")}>{&props.name}</div>
          </div>
          // Water Status
          <div class={classes!()}>
          </div>
          // Last Watered By
          <div class={classes!()}>
          </div>
        </div>
      </div>
    }
}
