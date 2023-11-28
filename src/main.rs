use doe::{args, Print};
use image::ImageFormat;
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
    use rayon::prelude::*;
    use std::{path::PathBuf, str::FromStr};
    let args = args!();
    if args.len() <= 1 {
        println!("Usage:");
        println!("generate-app-icons <icon_path>");
        println!("generate-app-icons resize 180::180 <icon_path>");
        return Ok(());
    }
    else if args.get(0).unwrap() == "resize"{
        let width = &args.get(1).unwrap().split_to_vec("::").as_slice()[0].parse::<u32>().unwrap_or(100);
        let height = &args.get(1).unwrap().split_to_vec("::").as_slice()[1].parse::<u32>().unwrap_or(100);
        let icon_path = args.get(2).unwrap();
        println!("resize {} to {}*{}", icon_path,width, height);
        resize_img_to_png(icon_path, (*width,*height), &format!("{}x{}.png", width, height));
    }
     else {
        let icon_path = &args[0];
        img_list::dir_paths()
        .par_iter()
        .for_each(|dir|{
            std::fs::create_dir_all(dir).expect("create directory failed");
        });

        img_list::icons()
        .par_iter()
        .for_each(|icon|{
            if icon.path.ends_with(".png") {
                resize_img_to_png(icon_path,icon.size,icon.path).unwrap();
                icon.path.push_back(" done!").println();
            }else if icon.path.ends_with(".ico") {
                resize_img_to_png(icon_path,(256,256),icon.path).unwrap();
                convert_to_ico(icon.path, icon.path).unwrap();
                icon.path.push_back(" done!").println();
            }else if icon.path.ends_with(".icns") {
                resize_img_to_png(icon_path,(1024,1024),icon.path).unwrap();
                convert_to_icns(icon.path,icon.path).unwrap();
                icon.path.push_back(" done!").println();
            }else if icon.path.ends_with(".bmp") {
                convert_to_bmp(icon_path, icon.path,icon.size).unwrap();
                icon.path.push_back(" done!").println();
            }
        });

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
fn resize_img_to_png(path: &str, size: (u32, u32), new_path: &str) -> Result<(), image::ImageError> {
    let img = image::open(path)?;
    let resized_img = img.resize_exact(size.0, size.1, image::imageops::FilterType::Gaussian);
    resized_img.save_with_format(new_path, ImageFormat::Png)?;
    Ok(())
}
#[allow(warnings)]
fn convert_to_bmp(icon_path: &str,new_icon_path:&str,size: (u32, u32)) -> Result<(), Box<dyn std::error::Error>>{
    let png_image = image::open(icon_path).expect("Failed to open PNG image");
    let resized_image = png_image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);
    resized_image.save_with_format(new_icon_path, ImageFormat::Bmp).expect("Failed to save BMP image");
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
