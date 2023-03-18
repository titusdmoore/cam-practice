use image::{ImageBuffer, Rgb};
use nokhwa::{
    nokhwa_initialize,
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    Camera,
};
use std::fs::File;

fn main() {
    nokhwa_initialize(|granted| {
        println!("User said {}", granted);
    });

    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    let camera_info = cameras.first().unwrap();

    let mut camera = Camera::new(camera_info.index().to_owned(), format).unwrap();
    camera.open_stream().unwrap();
    let buffer = camera.frame().unwrap();
    camera.stop_stream().unwrap();

    let vec_buff = nv12_to_rgb(
        buffer.buffer(),
        buffer.resolution().width() as usize,
        buffer.resolution().height() as usize,
    );

    image::save_buffer(
        "image.png",
        &vec_buff,
        buffer.resolution().width(),
        buffer.resolution().height(),
        image::ColorType::Rgb8,
    )
    .unwrap()
}

fn nv12_to_rgb(nv12_buffer: &[u8], width: usize, height: usize) -> Vec<u8> {
    // Calculate the size of the output RGB buffer
    let rgb_size = width * height * 3;

    // Create a new buffer to hold the RGB data
    let mut rgb_buffer = vec![0u8; rgb_size];

    // YUV to RGB conversion constants
    let yuv_constants = [
        298.082_ / 256.0,
        0.0,
        408.583_ / 256.0,
        298.082_ / 256.0,
        -100.291_ / 256.0,
        -208.120_ / 256.0,
        298.082_ / 256.0,
        516.411_ / 256.0,
        0.0,
    ];

    // Iterate over each pixel in the NV12 buffer
    for row in 0..height {
        for col in 0..width {
            let y = nv12_buffer[row * width + col] as f64;
            let u = nv12_buffer[(height + (row / 2) * width) + (col / 2) * 2] as f64 - 128.0;
            let v = nv12_buffer[(height + (row / 2) * width) + (col / 2) * 2 + 1] as f64 - 128.0;

            let r =
                (yuv_constants[0] * y + yuv_constants[1] * u + yuv_constants[2] * v).round() as u8;
            let g =
                (yuv_constants[3] * y + yuv_constants[4] * u + yuv_constants[5] * v).round() as u8;
            let b =
                (yuv_constants[6] * y + yuv_constants[7] * u + yuv_constants[8] * v).round() as u8;

            let pixel_offset = (row * width + col) * 3;
            rgb_buffer[pixel_offset] = r;
            rgb_buffer[pixel_offset + 1] = g;
            rgb_buffer[pixel_offset + 2] = b;
        }
    }

    rgb_buffer
}
