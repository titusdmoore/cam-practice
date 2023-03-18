use image::{ImageBuffer, Rgb};
use nokhwa::{
    nokhwa_initialize,
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    Camera,
    FormatDecoder
};
use std::{fs::File, io::Write};

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

    
    let mut vec_buff: Vec<u8> = Vec::new();
    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = buffer.decode_image::<RgbFormat>().unwrap();

    image.save("image.png").expect("unable to save image");
}
