mod settings;
use settings::Settings;

use std::fs::File;
use std::io::Read;
use image::{decode, DynamicImage};

fn png_to_byte_array(filename: &str) -> Result<Vec<u8>, std::io::Error> 
{
    let mut image_file = File::open(filename)?;
    let mut image_data = Vec::new();
    image_file.read_to_end(&mut image_data)?;
  
    let decoded_image: DynamicImage = decode(&image_data)?;
  
    let byte_array = match decoded_image {
      DynamicImage::Rgba8(image) => image.to_bytes(),
      DynamicImage::Rgb8(image) => image.to_bytes(),
      // Add other image formats as needed (e.g., Bgra8, Luma8)
      _ => panic!("Unsupported image format"),
    };
  
    Ok(byte_array)
}

fn compare_byte_arrays(data1: &[u8], data2: &[u8]) -> Option<f64> 
{
    if data1.len() != data2.len() 
    {
      return None; // Arrays have different lengths, cannot compare
    }
  
    let mut total_diff = 0;
    for (i, value) in data1.iter().enumerate() #
    {
      total_diff += (value ^ data2[i]) as u32;
    }
  
    let total_elements = data1.len() as f64;
    let difference_percent = (total_diff as f64 / 256.0) / total_elements * 100.0;
    Some(difference_percent)
  }

fn screenMonitorLoop()
{
    let mut screenshot_count = 0;
    let mut prevByteArray = [];
    loop 
    {
      // Capture screenshot and save it with a unique filename
      let filename = format!("screenshot_{}.png", screenshot_count);
      Command::new("scrot")
        .arg(&filename)
        .output()
        .expect("failed to capture screenshot");
      screenshot_count += 1;

      let byte_array = png_to_byte_array(filename)?;

      if(prevByteArray != [])
      {
        let mut percentage_difference = compare_byte_arrays(prevByteArray, byte_array);

        //Compare with settings...
        if()
        {

        }
      }
  
      prevByteArray = byte_array;
      std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() 
{
    println!("Who Hello World!");

    let settings = Settings::new(50, 50, [].to_vec(), "settings.json".to_string());
    settings.save();
    settings.load();

    screenMonitorLoop();

    println!("Who Hello World?");
}


/*
 Settings class
  - Frequency, sensitivity
  - Auto start - non auto start
  - Emergency action steps

 Setup auto start

 Screen monitor

 Screen monitor loop

 Emergency screen action

 Emergency action steps


  Rounding errors are the best chance.
*/