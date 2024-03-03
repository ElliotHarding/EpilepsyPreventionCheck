mod settings;
use settings::Settings;

fn main() {
    println!("Who Hello World!");

    let mut settings = match Settings::load() 
    {
        Ok(settings) => settings,
        Err(_) => {
            println!("Settings file not found, creating a new one...");

            Settings 
            {
                m_fileName: String::from("settings.json"),
                m_limitFrequency: 32,
                m_sensitivity: 50,
                m_emergencyActions: [], 
            }
        }
    };

    // Modify settings if needed
    // ...

    // Save settings back to file
    settings.save()?;

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
*/