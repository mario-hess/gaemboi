use gaemboi::GameBoyType;
use std::{error::Error, io::Read, path::Path};

pub fn extract_from_path(file_path: &String) -> Result<(GameBoyType, Vec<u8>), Box<dyn Error>> {
    let path = Path::new(file_path);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or("File has no valid extension")?;

    let gb_type = match extension {
        "gb" => GameBoyType::GameBoyClassic,
        "gbc" => GameBoyType::GameBoyColor,
        "gba" => GameBoyType::GameBoyAdvance,
        _ => return Err(format!("Unsupported file extension: {}", extension).into()),
    };

    let mut file = std::fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok((gb_type, data))
}
