use eframe::egui::{Color32, ColorImage};
use image::{DynamicImage, GenericImageView};

fn compute_low_pass_filter(grayscale_img: &DynamicImage, width: u32, height: u32) -> Vec<Color32> {
    let mut filtered_pixels = Vec::new();

    // Define a 5x5 Gaussian kernel (normalized)
    let gaussian_kernel: [[f32; 5]; 5] = [
        [
            1.0 / 273.0,
            4.0 / 273.0,
            7.0 / 273.0,
            4.0 / 273.0,
            1.0 / 273.0,
        ],
        [
            4.0 / 273.0,
            16.0 / 273.0,
            26.0 / 273.0,
            16.0 / 273.0,
            4.0 / 273.0,
        ],
        [
            7.0 / 273.0,
            26.0 / 273.0,
            41.0 / 273.0,
            26.0 / 273.0,
            7.0 / 273.0,
        ],
        [
            4.0 / 273.0,
            16.0 / 273.0,
            26.0 / 273.0,
            16.0 / 273.0,
            4.0 / 273.0,
        ],
        [
            1.0 / 273.0,
            4.0 / 273.0,
            7.0 / 273.0,
            4.0 / 273.0,
            1.0 / 273.0,
        ],
    ];

    for y in 2..height - 2 {
        for x in 2..width - 2 {
            let mut weighted_sum = 0.0;

            // Apply the kernel to the 5x5 neighborhood
            for ky in 0..5 {
                for kx in 0..5 {
                    let px: u32 = (x as i32 + kx as i32 - 2) as u32;
                    let py = (y as i32 + ky as i32 - 2) as u32;

                    let current_pixel = grayscale_img.get_pixel(px, py);
                    let intensity = current_pixel[0] as f32;
                    weighted_sum += gaussian_kernel[ky][kx] * intensity;
                }
            }

            // Clamp the result to [0, 255]
            let intensity = weighted_sum.min(255.0).max(0.0) as u8;
            filtered_pixels.push(Color32::from_gray(intensity));
        }
    }

    filtered_pixels
}

fn compute_laplace(grayscale_img: &DynamicImage, width: u32, height: u32) -> Vec<Color32> {
    let mut laplace_pixels = Vec::new();

    let laplace_filter: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]];

    for y in 1..width - 1 {
        for x in 1..height - 1 {
            let mut weighted_sum = 0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let current_pixel =
                        grayscale_img.get_pixel(x + kx as u32 - 1, y + ky as u32 - 1);
                    let pixel = current_pixel[0] as i32;
                    weighted_sum += pixel * laplace_filter[ky][kx];
                }
            }

            let intensity = weighted_sum.clamp(0, 255) as u8;
            laplace_pixels.push(Color32::from_gray(intensity as u8));
        }
    }

    laplace_pixels
}

fn compute_gaussian(grayscale_img: &DynamicImage, width: u32, height: u32) -> Vec<Color32> {
    let mut gaussian_pixels = Vec::new();

    // Define a 5x5 Gaussian kernel (normalized)
    let gaussian_kernel: [[f32; 5]; 5] = [
        [
            1.0 / 273.0,
            4.0 / 273.0,
            7.0 / 273.0,
            4.0 / 273.0,
            1.0 / 273.0,
        ],
        [
            4.0 / 273.0,
            16.0 / 273.0,
            26.0 / 273.0,
            16.0 / 273.0,
            4.0 / 273.0,
        ],
        [
            7.0 / 273.0,
            26.0 / 273.0,
            41.0 / 273.0,
            26.0 / 273.0,
            7.0 / 273.0,
        ],
        [
            4.0 / 273.0,
            16.0 / 273.0,
            26.0 / 273.0,
            16.0 / 273.0,
            4.0 / 273.0,
        ],
        [
            1.0 / 273.0,
            4.0 / 273.0,
            7.0 / 273.0,
            4.0 / 273.0,
            1.0 / 273.0,
        ],
    ];

    // Iterate over the image excluding borders
    for y in 2..height - 2 {
        for x in 2..width - 2 {
            let mut weighted_sum = 0.0;

            // Apply the kernel to the 5x5 neighborhood
            for ky in 0..5 {
                for kx in 0..5 {
                    let px = (x as i32 + kx as i32 - 2) as u32;
                    let py = (y as i32 + ky as i32 - 2) as u32;

                    let current_pixel = grayscale_img.get_pixel(px, py);
                    let intensity = current_pixel[0] as f32;
                    weighted_sum += gaussian_kernel[ky][kx] * intensity;
                }
            }

            // Clamp the result to [0, 255]
            let intensity = weighted_sum.min(255.0).max(0.0) as u8;
            gaussian_pixels.push(Color32::from_gray(intensity));
        }
    }
    gaussian_pixels
}

fn compute_sobel(grayscale_img: &DynamicImage, width: u32, height: u32) -> Vec<Color32> {
    let mut sobel_pixels = Vec::new();
    let kernel_x: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let kernel_y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut gx = 0;
            let mut gy = 0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let current_pixel: image::Rgba<u8> =
                        grayscale_img.get_pixel(x + kx as u32 - 1, y + ky as u32 - 1);
                    let pixel: i32 = current_pixel[0] as i32;
                    gx += kernel_x[ky][kx] * pixel;
                    gy += kernel_y[ky][kx] * pixel;
                }
            }
            let magnitude = ((gx * gx + gy * gy) as f64).sqrt().min(255.0).max(0.0) as u8;
            sobel_pixels.push(Color32::from_gray(magnitude));
        }
    }
    sobel_pixels
}

fn create_color_image(width: u32, height: u32, data: Vec<Color32>) -> ColorImage {
    ColorImage {
        size: [width as usize, height as usize],
        pixels: data,
    }
}

pub fn create_images_with_channels(
    image_bytes: &[u8],
) -> Option<(
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
    ColorImage,
)> {
    let img = image::load_from_memory(image_bytes).ok()?;

    let grayscale_img = img.grayscale();
    let img = img.to_rgba8();

    let mut red_pixels = Vec::new();
    let mut green_pixels = Vec::new();
    let mut blue_pixels = Vec::new();
    let mut original_pixels = Vec::new();
    let mut grayscale_pixels = Vec::new();

    for pixel in grayscale_img.pixels() {
        let intensity = pixel.2;
        let intensity = intensity[0];
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

    let (width, height) = grayscale_img.dimensions();

    let original_image = create_color_image(width, height, original_pixels);
    let red_image = create_color_image(width, height, red_pixels);
    let green_image = create_color_image(width, height, green_pixels);
    let blue_image = create_color_image(width, height, blue_pixels);
    let grayscale_image = create_color_image(width, height, grayscale_pixels);

    let sobel_pixels = compute_sobel(&grayscale_img, width, height);
    let sobel_image = create_color_image(width - 2, height - 2, sobel_pixels);

    let gaussian_pixels = compute_gaussian(&grayscale_img, width, height);
    let gaussian_image = create_color_image(width - 4, height - 4, gaussian_pixels);

    let laplace_pixels: Vec<Color32> = compute_laplace(&grayscale_img, width, height);
    let laplace_image = create_color_image(width - 2, height - 2, laplace_pixels);

    let low_pass_pixels: Vec<Color32> = compute_low_pass_filter(&grayscale_img, width, height);
    let low_pass_image = create_color_image(width - 4, height - 4, low_pass_pixels);

    Some((
        original_image,
        red_image,
        green_image,
        blue_image,
        grayscale_image,
        sobel_image,
        gaussian_image,
        laplace_image,
        low_pass_image,
    ))
}
