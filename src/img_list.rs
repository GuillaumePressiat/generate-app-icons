#[allow(warnings)]
pub struct IconImage<T: ToString> {
    pub path: T,
    pub size: (u32, u32),
}
#[allow(warnings)]
pub fn dir_paths() -> &'static [&'static str] {
    &[
        "./app_icons/Assets.xcassets/AppIcon.appiconset",
        "./app_icons/Assets.xcassets",
        "./app_icons/android/mipmap-mdpi",
        "./app_icons/android/mipmap-hdpi",
        "./app_icons/android/mipmap-xxxhdpi",
        "./app_icons/android/mipmap-xxhdpi",
        "./app_icons/android/mipmap-xhdpi",
        "./app_icons/android",
    ]
}
#[allow(warnings)]
pub fn icons() -> Vec<IconImage<&'static str>> {
    vec![
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/88.png",
            size: (88, 88),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/1024.png",
            size: (1024, 1024),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/76.png",
            size: (76, 76),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/60.png",
            size: (60, 60),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/48.png",
            size: (48, 48),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/216.png",
            size: (216, 216),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/64.png",
            size: (64, 64),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/58.png",
            size: (58, 58),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/167.png",
            size: (167, 167),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/72.png",
            size: (72, 72),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/172.png",
            size: (172, 172),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/29.png",
            size: (29, 29),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/100.png",
            size: (100, 100),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/114.png",
            size: (114, 114),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/128.png",
            size: (128, 128),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/512.png",
            size: (512, 512),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/16.png",
            size: (16, 16),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/120.png",
            size: (120, 120),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/256.png",
            size: (256, 256),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/20.png",
            size: (20, 20),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/32.png",
            size: (32, 32),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/180.png",
            size: (180, 180),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/57.png",
            size: (57, 57),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/80.png",
            size: (80, 80),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/55.png",
            size: (55, 55),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/196.png",
            size: (196, 196),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/40.png",
            size: (40, 40),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/87.png",
            size: (87, 87),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/50.png",
            size: (50, 50),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/144.png",
            size: (144, 144),
        },
        IconImage {
            path: "app_icons/Assets.xcassets/AppIcon.appiconset/152.png",
            size: (152, 152),
        },
        IconImage {
            path: "app_icons/appstore.png",
            size: (1024, 1024),
        },
        IconImage {
            path: "app_icons/android/mipmap-mdpi/ic_launcher.png",
            size: (48, 48),
        },
        IconImage {
            path: "app_icons/android/mipmap-hdpi/ic_launcher.png",
            size: (72, 72),
        },
        IconImage {
            path: "app_icons/android/mipmap-xxxhdpi/ic_launcher.png",
            size: (192, 192),
        },
        IconImage {
            path: "app_icons/android/mipmap-xxhdpi/ic_launcher.png",
            size: (144, 144),
        },
        IconImage {
            path: "app_icons/android/mipmap-xhdpi/ic_launcher.png",
            size: (96, 96),
        },
        IconImage {
            path: "app_icons/playstore.png",
            size: (512, 512),
        },
    ]
}
