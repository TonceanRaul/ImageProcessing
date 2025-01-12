use crate::image_processing::create_images_with_channels;
use eframe::egui::{self};
use egui_extras::RetainedImage;

pub struct MyApp {
    original: Option<RetainedImage>,
    red_channel: Option<RetainedImage>,
    green_channel: Option<RetainedImage>,
    blue_channel: Option<RetainedImage>,
    grayscale_image: Option<RetainedImage>,
    sobel_image: Option<RetainedImage>,
    gaussian_image: Option<RetainedImage>,
    laplace_image: Option<RetainedImage>,
    low_pass_image: Option<RetainedImage>,
}

impl Default for MyApp {
    fn default() -> Self {
        let image_bytes = include_bytes!("../lena_color.jpg");

        if let Some((original, red, green, blue, grayscale, sobel, gaussian, laplace, low_pass)) =
            create_images_with_channels(image_bytes)
        {
            Self {
                original: Some(RetainedImage::from_color_image("Original", original)),
                red_channel: Some(RetainedImage::from_color_image("Red Channel", red)),
                green_channel: Some(RetainedImage::from_color_image("Green Channel", green)),
                blue_channel: Some(RetainedImage::from_color_image("Blue Channel", blue)),
                grayscale_image: Some(RetainedImage::from_color_image("Grayscale", grayscale)),
                sobel_image: Some(RetainedImage::from_color_image("Sobel Filter", sobel)),
                gaussian_image: Some(RetainedImage::from_color_image("Gaussian Filter", gaussian)),
                laplace_image: Some(RetainedImage::from_color_image("Laplace Filter", laplace)),
                low_pass_image: Some(RetainedImage::from_color_image("Low Pass Filter", low_pass)),
            }
        } else {
            Self {
                original: None,
                red_channel: None,
                green_channel: None,
                blue_channel: None,
                grayscale_image: None,
                sobel_image: None,
                gaussian_image: None,
                laplace_image: None,
                low_pass_image: None,
            }
        }
    }
}

fn display_image(title: &str, channel: &Option<RetainedImage>, ctx: &egui::Context) {
    egui::Window::new(title).show(ctx, |ui| {
        if let Some(channel) = channel {
            channel.show(ui);
        } else {
            ui.label("Failed to load ".to_string() + title);
        }
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(original) = &self.original {
                ui.label("Original Image:");
                original.show(ui);
            } else {
                ui.label("Failed to load original image!");
            }
        });

        display_image("Red Channel", &self.red_channel, ctx);
        display_image("Green Channel", &self.green_channel, ctx);
        display_image("Blue Channel", &self.blue_channel, ctx);
        display_image("Grayscale", &self.grayscale_image, ctx);
        display_image("Sobel", &self.sobel_image, ctx);
        display_image("Gaussian", &self.gaussian_image, ctx);
        display_image("Laplace", &self.laplace_image, ctx);
        display_image("Low Pass Filter", &self.low_pass_image, ctx);
    }
}
