use std::mem::size_of_val;

use eframe::NativeOptions;
use egui::Color32;

fn main() {
    eframe::run_native(
        "draw",
        NativeOptions {
            centered: true,
            default_theme: eframe::Theme::Light,
            ..Default::default()
        },
        Box::new(|_| Box::new(App::default())),
    );
}

struct Stroke {
    points: Vec<egui::Pos2>,
    color: Color32,
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new(Color32::RED)
    }
}

impl Stroke {
    fn new(color: Color32) -> Self {
        Self {
            points: Vec::new(),
            color,
        }
    }

    fn paint(&self, ui: &mut egui::Ui) {
        for two_points in self.points.windows(2) {
            ui.painter()
                .line_segment(two_points.try_into().unwrap(), (1.0, self.color));
        }
    }
}

#[derive(Default)]
struct App {
    strokes: Vec<Stroke>,
    current_stroke: Stroke,
}

impl App {
    fn size_of_strokes(&self) -> usize {
        size_of_val(&*self.current_stroke.points)
            + self
                .strokes
                .iter()
                .map(|stroke| size_of_val(&*stroke.points))
                .sum::<usize>()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("controls here!");
                ui.separator();
                ui.label(format!(
                    "memory usage: {} MB",
                    self.size_of_strokes() as f32 / (1024.0 * 1024.0)
                ));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("draw here!");

            if ctx
                .input()
                .pointer
                .button_down(egui::PointerButton::Primary)
            {
                let pos = ctx.input().pointer.interact_pos().unwrap();
                self.current_stroke.points.push(pos);
                // println!("pos: {:?}, {}", pos, self.current_stroke.points.len());
            }

            if ctx.input().pointer.any_released()
            // bug in egui
            {
                let random_color = [Color32::RED, Color32::BLUE, Color32::GREEN, Color32::GOLD]
                    [((ui.input().time * 1000.0) as usize) % 4];
                let new_stroke =
                    std::mem::replace(&mut self.current_stroke, Stroke::new(random_color));
                self.strokes.push(new_stroke);
            }

            for stroke in &self.strokes {
                stroke.paint(ui);
            }
            self.current_stroke.paint(ui);
        });
    }
}
