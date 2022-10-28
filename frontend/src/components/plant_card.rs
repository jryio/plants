use chrono::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlantContainerProps {
    pub children: Children,
}

#[function_component(PlantContainer)]
pub fn plant_container(props: &PlantContainerProps) -> Html {
    html! {
      <div class={classes!("flex", "flex-row", "flex-wrap", "gap-4" )}>
        { props.children.clone() }
      </div>
    }
}

// TODO: Compute status of plant based on last water date
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PlantCardProps {
    pub name: String,
    pub image: String,
    pub room: String,
    pub water_frequency: usize,
    pub water_instructions: String,
    pub last_watered_date: DateTime<Utc>,
    pub last_watered_by: String,
}

#[function_component(PlantCard)]
pub fn plant_preview(props: &PlantCardProps) -> Html {
    // log::debug!("props = {:?}", props);
    let days_since_watered = props
        .last_watered_date
        .signed_duration_since(Utc::now())
        .num_days();
    let last_watered_emoji = {
        match days_since_watered as u64 {
            0..=3 => "😄",
            4..=7 => "😅",
            8..=14 => "🥵",
            15.. => "‼️☠️‼️",
        }
    };
    let last_watered_date_by = format!(
        "Last watered {} days ago by {}",
        days_since_watered, props.last_watered_by
    );
    html! {
      // Container
      <div class={classes!("flex", "flex-col","w-72", "h-72","rounded-md","bg-green-100", "hover:cursor-pointer")}>
        // Image
        <img
            alt="plant card image"
            src={props.image.clone()}
            class={classes!("flex-shrink-0","h-4/6","object-cover","self-start", "w-full", "max-w-full", "rounded-t-md", "bg-gray-400")}
        />
        <div class={classes!("p-2", "h-2/6")}>
          // Name
          <div class={classes!("flex-shrink-1", "flex-wrap", "overflow-hidden", "opactiy-100")}>
            <span class={classes!("text-lg", "font-medium")}>
              {format!("{} {}", last_watered_emoji, props.name)}
          </span>
          </div>
          // Last Watered Status
          <div class={classes!()}>
            {last_watered_date_by}
          </div>
        </div>
      </div>
    }
}
