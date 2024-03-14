mod settings;
use settings::Settings;

fn screenMonitorLoop()
{
    let mut screenshot_count = 0;
    loop 
    {
      // Capture screenshot and save it with a unique filename
      let filename = format!("screenshot_{}.png", screenshot_count);
      Command::new("scrot")
        .arg(&filename)
        .output()
        .expect("failed to capture screenshot");
      screenshot_count += 1;

        

  
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