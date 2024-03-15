mod settings;
use settings::Settings;
use std::process::Command;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;

fn imageFileToByteArray(filePath: String) -> Result<Vec<u8>, std::io::Error>
{
    println!("      imageFileToByteArray :: ...");

    let file = File::open(filePath)?;
    let mut reader = BufReader::new(file);

    println!("      imageFileToByteArray :: Found file");

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

    /*for byte in returnArray.iter() 
    {
        println!("imageFileToByteArray :: Value: {}", byte);
    }*/

    println!("      imageFileToByteArray :: Processed file");
    Ok(returnArray)
}

//Difference as a percentage
fn compareVecU8s(mut vector1: Vec<u8>, mut vector2: Vec<u8>) -> Option<f64>
{
    println!("      compareVecU8s        :: ");

    if vector1.len() != vector2.len()
    {
        println!("      compareVecU8s        :: Hard to comapre due to different lengths");
        println!("      compareVecU8s        ::  Len 1: {}", vector1.len());
        println!("      compareVecU8s        ::  Len 2: {}", vector2.len());

        if vector1.len() > vector2.len()
        {
            vector2.extend(vec![0; vector1.len() - vector2.len()]);
        }
        else
        {
            vector1.extend(vec![0; vector2.len() - vector1.len()]);
        }
    }

    let mut difference : f64 = 0.0;
    for (i, value) in vector1.iter().enumerate()    
    {
        difference += (value ^ vector2[i]) as f64;
    }

    println!("      compareVecU8s        :: difference: {}" , difference);

    let mut percentageDifference = ((difference as f64) / 256.0 / (vector1.len() as f64 * 100.0)) as f64;
    Some(percentageDifference)
}

fn screenMonitorLoop(settings: &Settings)
{
    println!("screenMonitorLoop    :: Starting loop");

    let mut iFrame = 0;
    let mut currentByteArray: Vec<u8> = Vec::new();
    loop
    {
        let filename = format!("screenshot_{}.png", iFrame);
        let folderName = "screenshots";

        /*if !fs::exists(folderName) 
        {
            fs::create_dir(folderName)?;
        }*/

        // Build the full path for the screenshot
        let screenShotPath = format!("{}/{}", folderName, filename);

        // Execute the scrot command with arguments
        let mut command = Command::new("scrot");
        command.arg(&screenShotPath);

        // Run the command and handle the result
        let output = command.output().expect("failed to execute scrot");

        println!("screenMonitorLoop    :: Screenshot status: {} ", output.status);

        let mut newByteArray = imageFileToByteArray(screenShotPath).ok().unwrap();
        if !currentByteArray.is_empty()
        {
            let mut percentage_difference = compareVecU8s(currentByteArray, newByteArray.clone()).unwrap();
            println!("screenMonitorLoop    :: Percentage Difference {}", percentage_difference);

            //Compare with settings...
            if (settings.m_sensitivity as f64) < percentage_difference
            {
                println!("screenMonitorLoop    :: ##########################Eplilepsy###################################");
            }
        }

        currentByteArray = newByteArray;
        iFrame += 1;
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