use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings
{
    m_limitFrequency: u32,
    m_sensitivity: u32,
    m_emergencyActions: Vec<EmergencyAction>,
    m_fileName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum EmergencyAction
{
    BlockScreen,
    MiniPopup,
    LogEmergency
}

impl Settings
{
    pub fn new(limitFrequency: u32, sensitivity: u32, emergencyActions: Vec<EmergencyAction>, fileName: String) -> Self 
    {
        Self {
            m_limitFrequency: limitFrequency,
            m_sensitivity: sensitivity,
            m_emergencyActions: emergencyActions,
            m_fileName: fileName.to_string(),
        }
    }

    pub fn load(&self) -> Result<Self, std::io::Error> 
    {
        let mut file = File::open(&self.m_fileName)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let settings: Self = serde_json::from_str(&contents)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), std::io::Error> 
    {
        let serialized = serde_json::to_string(&self)?;
        let mut file = OpenOptions::new().create(true).write(true).open(&self.m_fileName)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
