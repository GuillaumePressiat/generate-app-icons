use doe::args;
mod img_list;
#[allow(warnings)]
fn main() -> Result<(), image::ImageError> {
    use doe::*;
    generate()?;
    Ok(())
}
#[allow(warnings)]
fn generate() -> Result<(), image::ImageError> {
    use doe::{DebugPrint, Str};
    use std::{path::PathBuf, str::FromStr};
    let args = args!();
    if args.len() != 1 {
        println!("Usage:");
        println!("generate-app-icons <icon_path>");
        return Ok(());
    } else {
        let icon_path = &args[0];
        for dir in img_list::dir_paths() {
            std::fs::create_dir_all(dir).expect("create directory failed");
        }
        let img = image::open(icon_path)?;

        for icon in img_list::icons() {
            if icon.path.ends_with(".png") {
                let resized_img = img.resize_exact(
                    icon.size.0,
                    icon.size.1,
                    image::imageops::FilterType::Gaussian,
                );
                resized_img.save(icon.path)?;
            }else if icon.path.ends_with(".ico") {
                convert_to_ico("app_icons/Assets.xcassets/AppIcon.appiconset/256.png", icon.path).unwrap();
            }else if icon.path.ends_with(".icns") {
                convert_to_icns("app_icons/appstore.png",icon.path).unwrap();
            }
        }

        let contents_json = include_str!("./Contents.json");
        std::fs::write(
            "./app_icons/Assets.xcassets/AppIcon.appiconset/Contents.json",
            contents_json,
        )
        .expect("wrire  contents_json failed");
        println!("Generation completed successfully");
        return Ok(());
    }
    Ok(())
}
#[allow(warnings)]
fn get_img_size(path: &str) -> (u32, u32) {
    use image::GenericImageView;
    let img = image::open(path).expect("open error");
    let (width, height) = img.dimensions();
    (width, height)
}

#[allow(warnings)]
fn resize_img(path: &str, size: (u32, u32), new_path: &str) -> Result<(), image::ImageError> {
    let img = image::open(path)?;
    let resized_img = img.resize_exact(size.0, size.1, image::imageops::FilterType::Gaussian);
    resized_img.save(new_path)?;
    Ok(())
}

#[allow(warnings)]
fn convert_to_icns(png_path: &str, icns_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use icns::{IconFamily, IconType, Image};
    let file = std::io::BufReader::new(std::fs::File::open(png_path).unwrap());
    let image = Image::read_png(file).unwrap();
    let w = image.width();
    let h = image.height();
    let new_file = std::io::BufWriter::new(std::fs::File::create(icns_path).unwrap());
    let mut icon_family = IconFamily::new();
    icon_family.add_icon(&image).unwrap();
    icon_family.write(new_file).unwrap();
    Ok(())
}

#[allow(warnings)]
fn convert_to_ico(png_path: &str, ico_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::write;
    use std::fs::File;
    use std::io::Write;
    // Create a new, empty icon collection:
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    // Read a PNG file from disk and add it to the collection:
    let file = std::fs::File::open(png_path).unwrap();
    let image = ico::IconImage::read_png(file).unwrap();
    icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
    let file = std::fs::File::create(ico_path).unwrap();
    icon_dir.write(file).unwrap();
    Ok(())
}
