use bounce::prelude::*;
use gloo::console::debug;
use std::convert::TryFrom;
use strum::{IntoEnumIterator, VariantNames};
use yew::prelude::*;
use yew_form::{Field, File, Form, Select, TextArea};

use crate::components::input::Label;
use common::{
    location::Location,
    person::Person,
    plant::{Plant, PlantConvertError, PlantForm as Model},
};

pub enum Action {
    Noop,
    Update,
    Submit(Form<Model>),
    RequestReset,
    RequestStart,
    RequestLoading,
    RequestDone,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestState {
    None,
    Loading,
    Done,
}

impl Default for RequestState {
    fn default() -> Self {
        RequestState::None
    }
}

#[derive(Slice, PartialEq, Default)]
pub struct PlantState {
    pub submitted: bool,
    pub request: RequestState,
    pub update_counter: usize,
    pub convert_error: String,
}

impl Reducible for PlantState {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Submit(mut form) => match form.validate() {
                true => match Plant::try_from(form.model()) {
                    Ok(_) => Self {
                        submitted: true,
                        request: self.request.clone(),
                        update_counter: self.update_counter + 1,
                        convert_error: String::default(),
                    }
                    .into(),
                    Err(error) => {
                        let convert_error = match error {
                                PlantConvertError::Location(_) => format!(
                                    "The 'location' field is not one of {:?}",
                                    Location::VARIANTS,
                                ),
                                PlantConvertError::Birthday(_) => {
                                    String::from("The 'birthday' field is not a valid chrono Date object")
                                }
                                PlantConvertError::LastWateredBy(_) => format!(
                                    "The 'last watered by' field is not one of {:?}",
                                    Person::VARIANTS,
                                ),
                                PlantConvertError::LastWateredDate(_) =>
                                  String::from("The 'last watered date' field is not a valid chrono DateTime object")
                            };
                        Self {
                            submitted: false,
                            request: self.request.clone(),
                            update_counter: self.update_counter + 1,
                            convert_error,
                        }
                        .into()
                    }
                },
                false => Self {
                    submitted: false,
                    request: self.request.clone(),
                    update_counter: self.update_counter + 1,
                    convert_error: String::default(),
                }
                .into(),
            },
            Action::Update => Self {
                submitted: self.submitted,
                request: self.request.clone(),
                update_counter: self.update_counter + 1,
                convert_error: String::default(),
            }
            .into(),
            Action::RequestStart => Self {
                submitted: self.submitted,
                request: self.request.clone(),
                update_counter: self.update_counter,
                convert_error: String::default(),
            }
            .into(),
            Action::RequestReset => Self {
                submitted: self.submitted,
                request: RequestState::None,
                update_counter: self.update_counter,
                convert_error: String::default(),
            }
            .into(),
            Action::RequestLoading => Self {
                submitted: self.submitted,
                request: RequestState::Loading,
                update_counter: self.update_counter,
                convert_error: String::default(),
            }
            .into(),
            Action::RequestDone => Self {
                submitted: self.submitted,
                request: RequestState::Done,
                update_counter: self.update_counter,
                convert_error: String::default(),
            }
            .into(),

            Action::Noop => Self {
                submitted: self.submitted,
                request: self.request.clone(),
                update_counter: self.update_counter,
                convert_error: String::default(),
            }
            .into(),
        }
    }
}

#[function_component(PlantForm)]
pub fn plant_form_new() -> Html {
    // -- State
    let state: UseSliceHandle<PlantState> = use_slice::<PlantState>();
    let form = use_state(|| Form::new(Model::default()));

    // -- Options
    let mut location_options: Vec<Html> = Location::iter()
        .map(|loc| {
            html! {
              <option value={loc.to_string()}>
            {loc}
            </option>

            }
        })
        .collect();

    location_options.push(html! {
      <option value="" selected={true}>
      </option>
    });

    let mut person_options: Vec<Html> = Person::iter()
        .map(|p| {
            html! {
              <option value={p.to_string()} >
            {p.name()}
            </option>

            }
        })
        .collect();

    person_options.push(html! {
      <option value="" selected={true}>
      </option>
    });

    // -- Callbacks
    let on_submit = {
        let state = state.clone();
        let form = form.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            state.dispatch(Action::Submit((*form).clone()))
        })
    };

    let on_input = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Update))
    };

    // -- Classes
    let error_class = classes!(vec!["text-red-600"]);
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
    let mut ic = vec!["form-input"];
    ic.extend(base.clone());
    let input_class = classes!(ic);
    let mut sc = vec!["form-select"];
    sc.extend(base.clone());
    let select_class = classes!(sc);
    let mut tac = vec!["form-textarea", "resize-none"];
    tac.extend(base.clone());
    let textarea_class = classes!(tac);
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
            <form onsubmit={on_submit}>
                <Label label={"Name"} no_margin={true}>
                    <Field<Model>
                        form={(*form).clone()}
                        field_name="name"
                        autocomplete="off"
                        class={input_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("name")}
                    </div>

                </Label>

                <Label label={"Location"}>
                    <Select<Model>
                        form={(*form).clone()}
                        field_name="location"
                        autocomplete="off"
                        class={select_class.clone()}
                        class_invalid={error_class.clone()}
                        // oninput={on_input.clone()}
                    >
                        {location_options}
                    </Select<Model>>
                    <div class={error_class.clone()}>
                        {form.field_message("location")}
                    </div>
                </Label>

                <Label label={"Birthday"}>
                    <Field<Model>
                        form={(*form).clone()}
                        field_name="birthday"
                        input_type="date"
                        autocomplete="off"
                        class={input_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("birthday")}
                    </div>
                </Label>

                <Label label={"Watering frequency (days)"}>
                    <Field<Model>
                        form={(*form).clone()}
                        field_name="water_frequency"
                        input_type="number"
                        autocomplete="off"
                        class={input_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("water_frequency")}
                    </div>
                </Label>

                <Label label={"Watering instructions"}>
                    <TextArea<Model>
                        form={(*form).clone()}
                        field_name="water_instructions"
                        rows="3"
                        autocomplete="off"
                        class={textarea_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("water_instructions")}
                    </div>
                </Label>
                <Label label={"Photo"}>
                    <File<Model>
                        form={(*form).clone()}
                        field_name="image"
                        accept="image/*"
                        capture="environment"
                        class={classes!(base.clone())}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("image")}
                    </div>
                </Label>

                <Label label={"Last watered date"} no_margin={true}>
                    <Field<Model>
                        form={(*form).clone()}
                        field_name="last_watered_date"
                        input_type="datetime-local"
                        autocomplete="off"
                        class={input_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    />
                    <div class={error_class.clone()}>
                        {form.field_message("last_watered_date")}
                    </div>
                </Label>
                <Label label={"Last watered by"}>
                    <Select<Model>
                        form={(*form).clone()}
                        field_name="last_watered_by"
                        autocomplete="off"
                        class={select_class.clone()}
                        class_invalid={error_class.clone()}
                        oninput={on_input.clone()}
                    >
                        { person_options }
                    </Select<Model>>
                    <div class={error_class.clone()}>
                        {form.field_message("last_watered_by")}
                    </div>
                </Label>
                <hr class={hr_class.clone()} />
                <div class={error_class.clone()}>
                    {state.convert_error.clone()}
                </div>
                // Allow this button to trigger the default form submit event
                <button type="submit" class={submit_class}>
                  {"Submit"}
                </button>
            </form>
        </>
    }
}
