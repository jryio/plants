use super::input::Label;
use yew::prelude::*;

#[function_component(PlantNew)]
pub fn plant_new() -> Html {
    let base = vec![
        "block",
        "w-full",
        "rounded-md",
        "border-gray-300",
        "shadow-sm",
        "focus:border-green-300",
        "focus:ring",
        "focus:ring-green-300",
        "focus:ring-opacity-50",
    ];
    let mut input_class = vec!["form-input"];
    input_class.extend(base.clone());
    let mut select_class = vec!["form-select"];
    select_class.extend(base.clone());
    let mut textarea_class = vec!["form-textarea", "resize-none"];
    textarea_class.extend(base.clone());

    let hr_class = classes!("mt-8", "mb-8");
    let submit_class = classes!(vec![
        "block",
        "w-full",
        "h-12",
        "rounded-md",
        "border-gray-300",
        "shadow-sm",
        "bg-green-400",
        "text-white",
        "hover:bg-green-300",
        "focus:border-red-300",
        "focus:ring",
        "focus:ring-red-300",
        "focus:ring-opacity-50",
    ]);

    html! {
        <>
            <h1>{"🪴Add a new plant"}</h1>
            <form>
                <Label label={"Name"} no_margin={true}>
                    <input type="text" autocomplete="off" class={input_class.clone()} />
                </Label>

                <Label label={"Location"}>
                    <select class={select_class.clone()}>
                        <option>{ "Living Room" }</option>
                        <option>{ "Kitchen" }</option>
                        <option>{ "Dining Room" }</option>
                        <option>{ "Bedroom" }</option>
                    </select>
                </Label>

                // TODO: Handle this return type
                <Label label={"Birthday"}>
                    <input type="date" class={input_class.clone()} />
                </Label>

                <Label label={"Watering frequency (days)"}>
                    <input type="number" class={input_class.clone()} />
                </Label>

                <Label label={"Watering instructions"}>
                    <textarea rows=3 class={textarea_class.clone()}>
                    </textarea>
                </Label>
                <Label label={"Photo"}>
                    <input type="file" capture="environment" accept="image/*" class={base.clone()}/>
                </Label>
                <hr class={hr_class.clone()} />
                <h4>{"Optional"}</h4>
                <p>{"If the plant has been around for a while but hasn't been added yet. Include when you watered it last."}</p>
                <Label label={"Last watered date"} no_margin={true}>
                    <input type="date" class={input_class.clone()} />
                </Label>
                <Label label={"Last watered by"}>
                    <select class={select_class.clone()}>
                        <option>{"Magda"}</option>
                        <option>{"Jacob"}</option>
                    </select>
                </Label>
                <hr class={hr_class.clone()} />
                <button type="submit" class={submit_class}>{"Submit"}</button>
            </form>
        </>
    }
}
