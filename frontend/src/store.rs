use log::info;
use yewdux::prelude::{Changed, Reducer};

pub enum Action {
    ModalOpen,
    ModalClose,
    Noop,
}

#[derive(Clone)]
pub struct Store {
    pub is_modal_open: bool,
}

impl Reducer for Store {
    type Action = Action;
    fn new() -> Self {
        info!("Reducer -- NEW");
        Self {
            is_modal_open: false,
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::Noop => {
                false
            },
            Action::ModalOpen => {
                info!("Reducer -- ModalOpen");
                self.is_modal_open = true;
                true
            }
            Action::ModalClose => {
                info!("Reducer -- ModalClose");
                self.is_modal_open = false;
                true
            }
        }
    }
}
