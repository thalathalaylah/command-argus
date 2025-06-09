use directories::ProjectDirs;

fn main() {
    match ProjectDirs::from("com", "command-argus", "command-argus") {
        Some(proj_dirs) => {
            println!("Data directory: {:?}", proj_dirs.data_dir());
            println!("Config directory: {:?}", proj_dirs.config_dir());
            println!("Cache directory: {:?}", proj_dirs.cache_dir());
            
            let storage_path = proj_dirs.data_dir().join("commands.json");
            println!("Commands JSON file path: {:?}", storage_path);
        }
        None => {
            println!("Failed to get project directories");
        }
    }
}