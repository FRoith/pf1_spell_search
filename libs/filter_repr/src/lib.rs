pub trait FilterRepr {
    fn create_btn(&self, ui: &mut egui::Ui) -> Self;

    fn test(&self, spell: &str) -> bool;

    fn get_all() -> Vec<Self>
    where
        Self: Sized;

    fn some_filter(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum FilterState {
    None,
    Positive,
    Negative,
}

impl FilterState {
    pub fn get_color(&self) -> egui::Color32 {
        match self {
            FilterState::None => egui::Color32::TRANSPARENT,
            FilterState::Positive => egui::Color32::from_hex("#008800").unwrap(),
            FilterState::Negative => egui::Color32::from_hex("#880000").unwrap(),
        }
    }

    pub fn n(&self) -> Self {
        match self {
            FilterState::None => FilterState::Positive,
            FilterState::Positive => FilterState::Negative,
            FilterState::Negative => FilterState::None,
        }
    }

    pub fn p(&self) -> Self {
        match self {
            FilterState::None => FilterState::Negative,
            FilterState::Positive => FilterState::None,
            FilterState::Negative => FilterState::Positive,
        }
    }

    pub fn test(&self, spell: &str, value: &str) -> bool {
        match self {
            FilterState::None => true,
            FilterState::Positive => spell.to_lowercase().contains(&value.to_lowercase()),
            FilterState::Negative => !spell.to_lowercase().contains(&value.to_lowercase()),
        }
    }
}
