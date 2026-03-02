use eframe::egui;
use serde::{Deserialize, Serialize};
use arabic_reshaper::arabic_reshape;
use unicode_bidi::BidiInfo;

#[derive(Deserialize, Serialize, Clone)]
struct Ayah {
    number: u32,
    text: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Surah {
    id: u32,
    name: String,
    transliteration: String,
    verses: Vec<Ayah>,
}

#[derive(Deserialize, Serialize)]
struct QuranData {
    surahs: Vec<Surah>,
}

struct TarteelApp {
    quran: Option<QuranData>,
    current_tab: Tab,
    search_term: String,
    selected_surah_idx: Option<usize>,
}

#[derive(PartialEq)]
enum Tab { Dashboard, Quran }

impl TarteelApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        let data = std::fs::read_to_string("assets/quran.json").ok()
            .and_then(|s| serde_json::from_str::<QuranData>(&s).ok());

        Self {
            quran: data,
            current_tab: Tab::Dashboard,
            search_term: String::new(),
            selected_surah_idx: None,
        }
    }

    // Helper to fix Arabic rendering (RTL + Connected letters)
    fn fix_arabic(&self, text: &str) -> String {
        let reshaped = arabic_reshape(text);
        let bidi_info = BidiInfo::new(&reshaped, None);
        if let Some(para) = bidi_info.paragraphs.get(0) {
            let line = para.range.clone();
            return bidi_info.reorder_line(para, line);
        }
        reshaped
    }
}

impl eframe::App for TarteelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let gold = egui::Color32::from_rgb(212, 175, 55);
        let navy = egui::Color32::from_rgb(10, 15, 28);

        egui::CentralPanel::default().frame(egui::Frame::none().fill(navy)).show(ctx, |ui| {
            // Static Header
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading(egui::RichText::new("TARTEEL").color(gold).size(30.0).strong());
                ui.label("13 Ramadan 1447 AH");
            });

            ui.add_space(10.0);
            ui.separator();

            if let Some(idx) = self.selected_surah_idx {
                self.show_surah_reader(ui, idx, gold);
            } else {
                match self.current_tab {
                    Tab::Dashboard => self.show_dashboard(ui, gold),
                    Tab::Quran => self.show_quran_list(ui, gold),
                }
            }

            // Bottom Navigation
            self.draw_nav(ui, gold);
        });
    }
}

impl TarteelApp {
    fn show_dashboard(&self, ui: &mut egui::Ui, gold: egui::Color32) {
        let prayers = [("Fajr", "05:11"), ("Dhuhr", "12:24"), ("Asr", "15:45"), ("Maghrib", "18:05"), ("Isha", "19:20")];
        for (name, time) in prayers {
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(name).color(egui::Color32::WHITE).size(18.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(time).color(gold).size(20.0));
                    });
                });
            });
            ui.add_space(5.0);
        }
    }

    fn show_quran_list(&mut self, ui: &mut egui::Ui, gold: egui::Color32) {
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_term);
        });
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(data) = &self.quran {
                for (i, surah) in data.surahs.iter().enumerate() {
                    if surah.transliteration.to_lowercase().contains(&self.search_term.to_lowercase()) || self.search_term.is_empty() {
                        if ui.button(format!("{}. {}", surah.id, surah.transliteration)).clicked() {
                            self.selected_surah_idx = Some(i);
                        }
                    }
                }
            }
        });
    }

    fn show_surah_reader(&mut self, ui: &mut egui::Ui, idx: usize, gold: egui::Color32) {
        if ui.button("⬅ Back").clicked() { self.selected_surah_idx = None; return; }
        
        if let Some(data) = &self.quran {
            let surah = &data.surahs[idx];
            ui.heading(egui::RichText::new(&surah.transliteration).color(gold));
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for ayah in &surah.verses {
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());
                        // Important: Reshape and Reverse Arabic for proper RTL display
                        let display_text = self.fix_arabic(&ayah.text);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.label(egui::RichText::new(display_text).size(24.0).color(egui::Color32::WHITE));
                        });
                        ui.label(egui::RichText::new(format!("({})", ayah.number)).size(12.0).color(gold));
                    });
                }
            });
        }
    }

    fn draw_nav(&mut self, ui: &mut egui::Ui, gold: egui::Color32) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.current_tab == Tab::Dashboard, "HOME").clicked() {
                    self.current_tab = Tab::Dashboard;
                    self.selected_surah_idx = None;
                }
                ui.add_space(40.0);
                if ui.selectable_label(self.current_tab == Tab::Quran, "QURAN").clicked() {
                    self.current_tab = Tab::Quran;
                }
            });
        });
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("q_font".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/amiri_quran.ttf")));
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "q_font".to_owned());
    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Tarteel", options, Box::new(|cc| Box::new(TarteelApp::new(cc))))
}
