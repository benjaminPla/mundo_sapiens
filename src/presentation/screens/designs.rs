use crate::presentation::components;
use crate::presentation::tokens;

// temp
struct DesignRow {
    name: &'static str,
}
const DESIGN_ROWS: &'static [DesignRow] = &[
    DesignRow { name: "design_name_0" },
    DesignRow { name: "design_name_1" },
];

pub struct ScreenDesigns {}

impl ScreenDesigns {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        components::depth_panel(ui, |ui| {
            ui.heading("Designs");
            ui.add_space(tokens::SPACING_SMALL);
            for d in DESIGN_ROWS { ui.label(d.name); }
        });
    }
}
