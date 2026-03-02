use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quran {
    pub surahs: Vec<Surah>,
}

#[derive(Serialize, Deserialize)]
pub struct Surah {
    pub id: u32,
    pub name: String,
    pub ayahs: Vec<Ayah>,
}

#[derive(Serialize, Deserialize)]
pub struct Ayah {
    pub text: String,
    pub number: u32,
}

pub fn load_json() -> Quran {
    let data = std::fs::read_to_string("assets/quran.json").expect("Failed to load Quran");
    serde_json::from_str(&data).unwrap()
}
