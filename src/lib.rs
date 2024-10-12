use base64::engine::general_purpose;
use base64::Engine;
use image::imageops::FilterType::Nearest;
use image::RgbaImage;
use utils::image_utils::{get_hicon, icon_to_image};
use utils::process_utils::get_process_path;
use uwp_apps::{get_uwp_icon, get_uwp_icon_base64};
mod utils {
    pub mod image_utils;
    pub mod process_utils;
}
mod uwp_apps;
use image::ImageReader;

pub fn get_icon_by_process_id(process_id: u32) -> RgbaImage {
    let path = get_process_path(process_id).expect("Failed to get process path");
    println!("Path: {}", path);
    if path.contains("WindowsApps") {
        return get_uwp_icon(&path).expect("Failed to get UWP icon");
    } else {
        return get_icon_by_path(&path);
    }
}

pub fn get_icon_by_path(path: &str) -> RgbaImage {
    unsafe {
        let icon = get_hicon(path);
        icon_to_image(icon)
    }
}

pub fn get_icon_base64_by_process_id(process_id: u32) -> String {
    let path = get_process_path(process_id).expect("Failed to get process path");
    get_icon_base64_by_path(&path)
}
pub fn read_image_to_base64(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let image = ImageReader::open(path)?.with_guessed_format()?.decode()?;

    // check image dimension
    let width = image.width();
    let height = image.height();
    println!(
        "width: {}, height: {}, total: {}",
        width,
        height,
        width * height
    );
    // if image bigger than 4000x4000 abort

    // Resize the image to 256x256
    let resized = image.resize(256, 256, Nearest);

    // Store the resized image in a buffer
    let mut buffer = Vec::new();
    let _ = resized.write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageFormat::Png,
    );
    // Return the base64-encoded image
    let encoded_image = general_purpose::STANDARD.encode(&buffer);

    Ok(encoded_image)
}

pub fn get_icon_base64_by_path(path: &str) -> String {
    if path.contains("WindowsApps") {
        return get_uwp_icon_base64(path).expect("Failed to get UWP icon base64");
    }
    // check if path is image file
    if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
        let base_string = read_image_to_base64(path);

        match base_string {
            Ok(base_string) => {
                // println!("baseString: {}", base_string);
                return base_string;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        // println!("baseString: {}", base_string);
    }

    let icon_image = get_icon_by_path(path);
    let mut buffer = Vec::new();
    icon_image
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .unwrap();
    general_purpose::STANDARD.encode(buffer)
}
