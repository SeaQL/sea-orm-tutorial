use eframe::{
    egui::{
        self, Button, Color32, CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Label,
        Layout, RichText, ScrollArea, Separator, TextStyle, TopBottomPanel, Vec2,
    },
    epi, run_native, NativeOptions,
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

fn main() {
    let app = TodoApp {
        username: "🚀 Not Connected".to_owned(),
        url: "http://127.0.0.1:4343".to_owned(),
        list: Vec::default(),
        text_buffer: String::default(),
    };
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 960.));

    run_native(Box::new(app), native_options);
}

impl epi::App for TodoApp {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_font(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(&self.username));
            ui.add_space(PADDING);
            let ui_separator = Separator::default().spacing(20.);
            ui.add(ui_separator);

            ui.text_edit_singleline(&mut self.text_buffer);
            if ui
                .button(RichText::new("Add TODO").color(Color32::RED))
                .clicked()
            {
                let text_buffer = &self.text_buffer;
                if !text_buffer.is_empty() {
                    self.list.push(text_buffer.to_owned());
                    self.text_buffer.clear();
                }
            }
            let ui_separator = Separator::default().spacing(50.);
            ui.add(ui_separator);

            ScrollArea::vertical()
                .always_show_scroll(false)
                .show(ui, |ui| {
                    for todo in &self.list {
                        ui.add_space(PADDING);
                        ui.colored_label(CYAN, todo);
                        ui.add_space(PADDING);
                        ui.add(Separator::default());
                        ui.add_space(PADDING);
                        ui.add_space(PADDING);
                    }
                });

            TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.);
                    ui.add(Label::new(
                        RichText::new("Apache-2.0, 2022. SeaQL").monospace(),
                    ));
                })
            })
        });
    }

    fn name(&self) -> &str {
        "SeaORM TODO List"
    }
}

pub struct TodoApp {
    pub username: String,
    pub url: String,
    pub list: Vec<String>,
    pub text_buffer: String,
}

impl TodoApp {
    fn configure_font(&self, ctx: &CtxRef) {
        // Psuedocode for font creation
        // 1. Create the FontDef object
        // 2. Load up the font
        // 3. Set the size of different text styles
        // 4. Load the font using the context object

        let mut font_def = FontDefinitions::default();

        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.5));

        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.));

        font_def
            .family_and_size
            .insert(TextStyle::Small, (FontFamily::Proportional, 10.));

        font_def
            .family_and_size
            .insert(TextStyle::Small, (FontFamily::Proportional, 10.));

        ctx.set_fonts(font_def);
    }
}
