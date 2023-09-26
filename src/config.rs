pub struct Config {
    pub file_path: Option<String>,
    pub tiletable_enable: bool,
    pub tilemaps_enable: bool,
    pub tilemap_9800_enable: bool,
    pub tilemap_9c00_enable: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Option<Self> {
        if args.len() < 2 {
            return None;
        }

        let mut file_path = None;

        for arg in args {
            if !arg.starts_with('-') && !arg.contains('/') {
                file_path = Some("roms/".to_owned() + arg)
            }
        }

        let tiletable_enable = args.contains(&"-t".to_string());
        let tilemaps_enable = args.contains(&"-m".to_string());
        let tilemap_9800_enable = args.contains(&"-m1".to_string());
        let tilemap_9c00_enable = args.contains(&"-m2".to_string());

        Some(Self {
            file_path,
            tiletable_enable,
            tilemaps_enable,
            tilemap_9800_enable,
            tilemap_9c00_enable,
        })
    }
}
