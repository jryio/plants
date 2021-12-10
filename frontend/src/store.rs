use bounce::*;
use yew::Reducible;

#[derive(Debug)]
pub enum Action {
    None,
    ModalOpen,
    ModalClose,
}

#[derive(Slice, PartialEq, Default)]
pub struct Store {
    pub is_modal_open: bool,
}

impl Reducible for Store {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::ModalOpen => Self {
                is_modal_open: true,
            }
            .into(),
            Action::ModalClose => Self {
                is_modal_open: false,
            }
            .into(),
            Action::None => self,
        }
    }
}
