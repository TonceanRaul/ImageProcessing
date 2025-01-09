use eframe::egui::{self, Color32, ColorImage};
use egui_extras::RetainedImage;
use image::{DynamicImage, GenericImageView, GrayImage};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Image Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    original: Option<RetainedImage>,
    red_channel: Option<RetainedImage>,
    green_channel: Option<RetainedImage>,
    blue_channel: Option<RetainedImage>,
    grayscale_image: Option<RetainedImage>,
}

impl Default for MyApp {
    fn default() -> Self {
        let image_bytes = include_bytes!("../lena_color.jpg");

        if let Some((original, red, green, blue, grayscale)) =
            create_images_with_channels(image_bytes)
        {
            Self {
                original: Some(RetainedImage::from_color_image("Original", original)),
                red_channel: Some(RetainedImage::from_color_image("Red Channel", red)),
                green_channel: Some(RetainedImage::from_color_image("Green Channel", green)),
                blue_channel: Some(RetainedImage::from_color_image("Blue Channel", blue)),
                grayscale_image: Some(RetainedImage::from_color_image("Grayscale", grayscale)),
            }
        } else {
            Self {
                original: None,
                red_channel: None,
                green_channel: None,
                blue_channel: None,
                grayscale_image: None,
            }
        }
    }
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

        egui::Window::new("Red Channel").show(ctx, |ui| {
            if let Some(red_channel) = &self.red_channel {
                red_channel.show(ui);
            } else {
                ui.label("Failed to load red channel!");
            }
        });

        egui::Window::new("Green Channel").show(ctx, |ui| {
            if let Some(green_channel) = &self.green_channel {
                green_channel.show(ui);
            } else {
                ui.label("Failed to load green channel!");
            }
        });

        egui::Window::new("Blue Channel").show(ctx, |ui| {
            if let Some(blue_channel) = &self.blue_channel {
                blue_channel.show(ui);
            } else {
                ui.label("Failed to load blue channel!");
            }
        });

        egui::Window::new("Grayscale").show(ctx, |ui| {
            if let Some(grayscale_image) = &self.grayscale_image {
                grayscale_image.show(ui);
            } else {
                ui.label("Failed to load grayscale image!");
            }
        });
    }
}

fn create_images_with_channels(
    image_bytes: &[u8],
) -> Option<(ColorImage, ColorImage, ColorImage, ColorImage, ColorImage)> {
    let img = image::load_from_memory(image_bytes).ok()?;
    let (width, height) = img.dimensions();

    let grayscale_img = img.grayscale();
    let img = img.to_rgba8();

    let mut red_pixels = Vec::new();
    let mut green_pixels = Vec::new();
    let mut blue_pixels = Vec::new();
    let mut original_pixels = Vec::new();
    let mut grayscale_pixels = Vec::new();

    for (y, x, pixel) in grayscale_img.pixels() {
        let intensity = pixel[0];
        grayscale_pixels.push(Color32::from_gray(intensity));
    }

    for pixel in img.pixels() {
        let [r, g, b, a] = pixel.0;

        // Add to respective channels
        red_pixels.push(Color32::from_rgba_premultiplied(r, 0, 0, a));
        green_pixels.push(Color32::from_rgba_premultiplied(0, g, 0, a));
        blue_pixels.push(Color32::from_rgba_premultiplied(0, 0, b, a));
        original_pixels.push(Color32::from_rgba_premultiplied(r, g, b, a));
    }

    let original_image = ColorImage {
        size: [width as usize, height as usize],
        pixels: original_pixels,
    };

    let red_image = ColorImage {
        size: [width as usize, height as usize],
        pixels: red_pixels,
    };

    let green_image = ColorImage {
        size: [width as usize, height as usize],
        pixels: green_pixels,
    };

    let blue_image = ColorImage {
        size: [width as usize, height as usize],
        pixels: blue_pixels,
    };

    let grayscale_image = ColorImage {
        size: [width as usize, height as usize],
        pixels: grayscale_pixels,
    };

    Some((
        original_image,
        red_image,
        green_image,
        blue_image,
        grayscale_image,
    ))
}
