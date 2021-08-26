use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn image_effect(img: &[u8], option: &str) -> Option<Box<[u8]>> {
    match option {
        "monochrome" => Some(monochrome(img)),
        "rotate_90"=>Some(rotate_90(img)),
        "rotate_180"=>Some(rotate_180(img)),
        "rotate_270"=>Some(rotate_270(img)),
        "grayscale"=>Some(grayscale(img)),
        _ => {
            console::log_1(&"unknown option".into());
            None
        }
    }
}

#[wasm_bindgen]
pub fn digest(str: &str) -> String {
    use web_sys::console;

    console::log_1(&str.into());
    let digest = md5::compute(str);
    let res = format!("{:x}", digest);
    res
}

#[wasm_bindgen]
pub fn monochrome(_array: &[u8]) -> Box<[u8]> {
    let img = get_image_as_base64(_array);
    img
    //return append_img(img);
}

fn get_image_as_base64(img: &[u8]) -> Box<[u8]> {
    let grey_img = image::load_from_memory(&img).unwrap().grayscale();
    let mut wr = Vec::new();
    grey_img
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}

#[wasm_bindgen]
pub fn rotate_90(img: &[u8]) -> Box<[u8]> {
    let img_90 = image::load_from_memory(&img).unwrap().rotate90();
    let mut wr = Vec::new();
    img_90
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}

#[wasm_bindgen]
pub fn rotate_180(img:&[u8])->Box<[u8]>{
    let img_180 = image::load_from_memory(&img).unwrap().rotate180();
    let mut wr = Vec::new();
    img_180
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}

#[wasm_bindgen]
pub fn rotate_270(img:&[u8])->Box<[u8]>{
    let img_270 = image::load_from_memory(&img).unwrap().rotate270();
    let mut wr = Vec::new();
    img_270
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}

#[wasm_bindgen]
pub fn resize(img:&[u8], height:u32, width:u32)->Box<[u8]>{
    let resized_img = image::load_from_memory(&img).unwrap().resize(width, height, image::imageops::FilterType::Gaussian);
    let mut wr = Vec::new();
    resized_img
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}


#[wasm_bindgen]
pub fn blur(img:&[u8], sigma:f32)->Box<[u8]>{
    let blur_img=image::load_from_memory(&img).unwrap().blur(sigma);
    let mut wr = Vec::new();
    blur_img
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}

#[wasm_bindgen]
pub fn grayscale(img:&[u8])->Box<[u8]>{
    let grayscale_img=image::load_from_memory(&img).unwrap().grayscale();
    let mut wr = Vec::new();
    grayscale_img
        .write_to(&mut wr, image::ImageOutputFormat::Png)
        .unwrap();

    let a = wr.into_boxed_slice();
    return a;
}
