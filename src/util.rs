fn toggle_ui(ui: &mut egui::Ui, on: &mut bool, text_off: &str, text_on: &str) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() || response.secondary_clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );

        let center = egui::pos2((rect.left() + rect.right()) / 2.0, rect.center().y);
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            if *on { text_off } else { text_on },
            egui::FontId::monospace(12.0),
            visuals.text_color(),
        );
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on, "OR", "AND")
}
