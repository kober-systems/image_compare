#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::ColorImage;
use image::DynamicImage;
use image_compare::*;

#[derive(Default)]
struct App {
    img1: DynamicImage,
    img2: DynamicImage,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            let height = ui.available_height();

            ui.allocate_ui_at_rect(
                egui::Rect::from_min_size(
                    egui::Pos2::new(0.0, 0.0),
                    egui::vec2(width * 0.5, height * 0.5),
                ),
                |mut ui| {
                    let rect = ui.max_rect();
                    ui.painter()
                        .rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 150, 150));
                    ui.label("original");
                    show_dynamic_image(&self.img1, width * 0.5, height * 0.45, &mut ui, ctx);
                },
            );
            ui.allocate_ui_at_rect(
                egui::Rect::from_min_size(
                    egui::Pos2::new(width * 0.5, 0.0),
                    egui::vec2(width * 0.5, height * 0.5),
                ),
                |mut ui| {
                    let rect = ui.max_rect();
                    ui.painter()
                        .rect_filled(rect, 0.0, egui::Color32::from_rgb(0, 255, 150));
                    ui.label("compared");
                    show_dynamic_image(&self.img2, width * 0.5, height * 0.45, &mut ui, ctx);
                },
            );
            ui.allocate_ui_at_rect(
                egui::Rect::from_min_size(
                    egui::Pos2::new(0.0, height * 0.5),
                    egui::vec2(width, height * 0.5),
                ),
                |mut ui| {
                    let rect = ui.max_rect();
                    ui.painter()
                        .rect_filled(rect, 0.0, egui::Color32::from_rgb(50, 25, 250));
                    ui.label("result");
                    show_dynamic_image(
                        &compare_images(&self.img1, &self.img2),
                        width,
                        height * 0.45,
                        &mut ui,
                        ctx,
                    );
                },
            );
        });
    }
}

fn show_dynamic_image(
    img: &DynamicImage,
    width: f32,
    height: f32,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
) -> egui::Response {
    match dynamic_image_to_egui(img) {
        Ok(img) => {
            ui.allocate_ui_with_layout(
                egui::Vec2::new(width, height),
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.add(
                        egui::Image::new(&ctx.load_texture(
                            "result",
                            img,
                            egui::TextureOptions::default(),
                        ))
                        .max_width(width)
                        .max_height(height),
                    )
                },
            )
            .response
        }
        Err(e) => ui.label(e),
    }
}

fn dynamic_image_to_egui(img: &DynamicImage) -> Result<egui::ColorImage, String> {
    let img_data = img.to_rgba8();

    Ok(ColorImage::from_rgba_unmultiplied(
        [
            img.width().try_into().or(Err("Problem with sizing"))?,
            img.height().try_into().or(Err("Problem with the height"))?,
        ],
        &img_data,
    ))
}

fn main() -> Result<(), anyhow::Error> {
    let args = options::Args::parse();
    let app = App {
        img1: image::open(args.img1)?,
        img2: image::open(args.img2)?,
    };

    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(app)
        }),
    )
    .expect("Something went wrong with egui");

    Ok(())
}
