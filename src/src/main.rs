mod settings;
use settings::Settings;
use std::process::Command;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;

/*
use std::io::Cursor;
use image::{open, ImageOutputFormat};

fn png_to_byte_array(filename: &str) -> Result<Vec<u8>, std::io::Error> 
{
    // Open the PNG image
    let img = open(filename)?;

    // Create a cursor to hold the image data in memory
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    // Serialize the image back to PNG format and write it to the cursor
    img.write_to(&mut cursor, ImageOutputFormat::Png)?;

    // Now "buffer" contains the image data as a byte array
    let image_bytes = buffer.as_slice();
  
    Ok(image_bytes)

    Explore via non errors
}*/




fn imageFileToByteArray(filePath: String) -> Result<Vec<u8>, std::io::Error> //Option<Vec<u8>>//Result<Vec<u8>, std::io::Error>
{
    let file = File::open(filePath)?;
    let mut reader = BufReader::new(file);

    // Define a buffer to hold the read bytes
    let mut buffer = [0u8; 1024]; // Adjust buffer size as needed

    let mut returnArray: Vec<u8> = Vec::new();

    // Read bytes into the buffer in a loop
    loop {
        // Read bytes from the buffer reader
        let bytes_read = reader.read(&mut buffer)?;

        // Check if end of file (EOF) is reached (0 bytes read)
        if bytes_read == 0 
        {
            break;
        }

        for byte in &buffer[..bytes_read] 
        {
            returnArray.push(*byte);
        }
    }

    Ok(returnArray)
}

//Difference as a percentage
fn compareVecU8s(vector1: &Vec<u8>, vector2: &Vec<u8>) -> Option<f64>
{
    let mut difference = 0;
    for (i, value) in vector1.iter().enumerate() 
    {
        difference += value ^ vector2[i];
    }

    let mut percentageDifference = (difference / 256 / (vector1.len() as u8 * 100)) as f64;
    Some(percentageDifference)
}

fn screenMonitorLoop(settings: &Settings)
{
    let mut screenshot_count = 0;
    let mut currentByteArray: Vec<u8> = Vec::new();
    loop
    {
      // Capture screenshot and save it with a unique filename
      let filename = format!("screenshot_{}.png", screenshot_count);
      Command::new("scrot")
        .arg(&filename)
        .output()
        .expect("failed to capture screenshot");
      screenshot_count += 1;

      let newByteArray = imageFileToByteArray(filename).ok().unwrap();

      if currentByteArray.is_empty()
      {
        let mut percentage_difference = compareVecU8s(&currentByteArray, &newByteArray).unwrap();
        println!("{}", percentage_difference);

        //Compare with settings...
        if (settings.m_sensitivity as f64) < percentage_difference
        {
            println!("Eplilepsy");
        }
      }
  
      currentByteArray = newByteArray;
      std::thread::sleep(std::time::Duration::from_secs(settings.m_limitFrequency as u64));
    }
}

fn main() 
{
    println!("Who Hello World!");

    let settings = Settings::new(1, 1, [].to_vec(), "settings.json".to_string());
    settings.save();
    settings.load();

    screenMonitorLoop(&settings);

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