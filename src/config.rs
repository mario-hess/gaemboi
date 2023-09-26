pub struct Config {
    pub file_path: String,
    pub tiletable_enable: bool,
    pub tilemaps_enable: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Option<Self> {
        if args.len() < 2 {
            return None;
        }

        let file_path = "roms/".to_owned() + &args[1];
        let tiletable_enable = args.contains(&"--table".to_string());
        let tilemaps_enable = args.contains(&"--maps".to_string());

        Some(Self {
            file_path: file_path.to_string(),
            tiletable_enable,
            tilemaps_enable,
        })
    }
}
