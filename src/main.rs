use nokhwa::{
    nokhwa_initialize,
    pixel_format::{FormatDecoder, RgbFormat},
    query,
    utils::{ApiBackend, FrameFormat, RequestedFormat, RequestedFormatType},
    Camera,
};

fn main() {
    nokhwa_initialize(|granted| {
        println!("User said {}", granted);
    });

    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    let camera_info = cameras.first().unwrap();

    let mut camera = Camera::new(camera_info.index().to_owned(), format).unwrap();
    camera.set_frame_format(FrameFormat::RAWRGB).unwrap();

    
    camera.open_stream().unwrap();
    let buffer = camera.frame().unwrap();
    camera.stop_stream().unwrap();


    // println!("{:?}", buffer.buffer());

    // Convert raw feed from frame to raw image
    // let raw_image = <RgbFormat as FormatDecoder>::write_output(
    //     FrameFormat::MJPEG,
    //     buffer.resolution(),
    //     buffer.buffer(),
    // )
    // .unwrap();

    let decoded = buffer.decode_image::<RgbFormat>().unwrap();

    image::save_buffer(
        "img.png",
        &decoded,
        buffer.resolution().width(),
        buffer.resolution().height(),
        image::ColorType::Rgb8,
    )
    .unwrap();

    // match buffer.decode_image::<RgbFormat>() {
    //     Ok(img_buff) => {
    //         let mut file = File::create("buffer.txt").unwrap();
    //         file.write_all(&img_buff).unwrap();

    //         let image: ImageBuffer<Rgb<u8>, Vec<u8>> = img_buff;

    //         match image.save("image.png") {
    //             Ok(_) => {}
    //             Err(e) => println!("{}", e),
    //         }
    //     }
    //     Err(e) => println!("{}", e),
    // }
}
