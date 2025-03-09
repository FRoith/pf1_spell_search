pub trait FilterRepr {
    fn create_btn(&self, ui: &mut egui::Ui) -> Self;

    fn test(&self, spell: &str) -> bool;

    fn get_all() -> Vec<Self>
    where
        Self: Sized;

    fn some_filter(&self) -> bool;
}
