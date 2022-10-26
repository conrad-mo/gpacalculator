#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CGPA Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    enabled: bool,
    assignments_str: String,
    quizzes_str: String,
    total: u32,
    toggle31: bool,
    quizweight: f32,
    assweight: f32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            toggle31: true,
            quizzes_str: "".to_owned(),
            assignments_str: "".to_owned(),
            enabled: true,
            total: 0,
            quizweight: 0.0,
            assweight: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CGPA Calculator");
            //ui.horizontal(|ui| {
                //ui.label("Your name: ");
                //ui.text_edit_singleline(&mut self.name);
            //});
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0; // remove spacing between widgets
                ui.radio_value(&mut self.toggle31, true, "MATA31");
                ui.radio_value(&mut self.toggle31, false, "MATA67");
            });
            ui.add(egui::TextEdit::singleline(&mut self.assignments_str).hint_text("Number of Assignments"));
            ui.add(egui::TextEdit::singleline(&mut self.quizzes_str).hint_text("Number of Quizzes"));
            if ui.button("Ready").clicked(){
                self.enabled = false;
                let _assignments_str: i32 = self.assignments_str.trim().parse().expect("Please type a number!");
                let _quizzes_str: i32 = self.quizzes_str.trim().parse().expect("Please type a number!");
            }
            ui.label(format!("Assignment Grade"));
            ui.label(format!("Assignment Weight"));
        });
    }
}