use doe::args;

mod img_list;
#[allow(warnings)]
fn main() -> Result<(), image::ImageError> {
    use doe::{DebugPrint, Str};
    use std::{path::PathBuf, str::FromStr};
    let args = args!();
    if args.len() != 1{
        println!("Usage:");
        println!("generate-app-icons <icon_path>");
        return Ok(());
    }else{
        let icon_path = &args[0];
        for dir in img_list::dir_paths(){
            std::fs::create_dir_all(dir).expect("create directory failed");
        }
        let img = image::open(icon_path)?;

        for icon in img_list::icons(){
            let resized_img = img.resize_exact(icon.size.0, icon.size.1, image::imageops::FilterType::Gaussian);
            resized_img.save(icon.path)?;
        }

        let contents_json = include_str!("./Contents.json");
        std::fs::write("./app_icons/Assets.xcassets/AppIcon.appiconset/Contents.json", contents_json).expect("wrire  contents_json failed");
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
