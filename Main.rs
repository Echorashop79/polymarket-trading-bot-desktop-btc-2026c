use std::fs;
use std::path::Path;
use std::time::Duration;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use rand::seq::SliceRandom;
use structopt::StructOpt;
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFI};
use widestring::WideCString;

/// Command-line arguments
#[derive(StructOpt)]
struct Cli {
    /// Folder containing wallpaper images
    #[structopt(short = "f", long = "folder", default_value = "C:\\Users\\Public\\Pictures\\Wallpapers")]
    folder: String,

    /// Change interval in minutes
    #[structopt(short = "i", long = "interval", default_value = "60")]
    interval: u64,

    /// Verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

/// Wallpaper changer struct
struct WallpaperChanger {
    images: Vec<String>,
    log_file: String,
}

impl WallpaperChanger {
    fn new(folder: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let images = Self::get_valid_images(folder)?;
        if images.is_empty() {
            return Err("No valid images found in the specified folder".into());
        }

        Ok(Self {
            images,
            log_file: "wallpaper_changer.log".to_string(),
        })
    }

    fn get_valid_images(folder: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let supported_extensions = [".jpg", ".jpeg", ".png", ".bmp"];
        let mut images = Vec::new();

        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if supported_extensions.iter().any(|e| e == &ext.to_string_lossy().to_lowercase()) {
                images.push(path.to_string_lossy().to_string());
            }
                }
            }
        }

        Ok(images)
    }

    fn set_wallpaper(&self, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Convert path to wide string for Windows API
        let wide_path = WideCString::from_str(image_path)?;

        unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                wide_path.as_ptr() as *mut _,
                SPIF_UPDATEINIFI,
            );
        }

        self.log_action(format!("Wallpaper changed to: {}", image_path))
    }

    fn change_to_random(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        if let Some(image) = self.images.choose(&mut rng) {
            self.set_wallpaper(image)?;
            if self.verbose {
                self.display_info(image)?;
            }
            Ok(())
        } else {
            Err("No images available".into())
        }
    }

    fn display_info(&self, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        match image::open(image_path) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let file_size = fs::metadata(image_path)?.len();

                println!("======================================");
                println!("CURRENT WALLPAPER INFO:");
                println!("Path: {}", image_path);
                println!("Resolution: {}x{}", width, height);
                println!("File size: {:.2} KB", file_size as f64 / 1024.0);
                println!("Format: {}", img.color().to_string());
                println!("====================================");
            }
            Err(e) => println!("Error reading image info: {}", e),
        }
        Ok(())
    }

    fn log_action(&self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] {}", timestamp, message);

        fs::write(&self.log_file, log_entry + "
")?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    println!("Wallpaper Changer started!");
    println!("Folder: {}", cli.folder);
    println!("Interval: {} minutes", cli.interval);
    println!("Press Ctrl+C to stop...");
    println!("------------------------------------");

    let changer = WallpaperChanger::new(&cli.folder)?;

    loop {
        changer.change_to_random()?;

        std::thread::sleep(Duration::from_secs(cli.interval * 60));
    }
}
