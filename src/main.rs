mod cpu;

use crate::cpu::CPU;
use eframe::egui;
use rand;

// fn main() {
//     let mut cpu = CPU::new();
//     cpu.run();

// }

fn example() -> egui::ColorImage {
    let width = 128;
    let height = 64;
    let mut img = egui::ColorImage::new([width, height], egui::Color32::TRANSPARENT);
    for y in 0..height {
        for x in 0..width {
            let h = x as f32 / width as f32;
            let s = 1.0;
            let v = 1.0;
            let a = y as f32 / height as f32;
            img[(x, y)] = egui::ecolor::Hsva {
                h: h + rand::random::<f32>(),
                s,
                v,
                a,
            }
            .into();
        }
    }
    img
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
            let tex = ui
                .ctx()
                .load_texture("my-image", example(), Default::default());
            ui.image(&tex, [640.0, 480.0]);
        });
    })
}
