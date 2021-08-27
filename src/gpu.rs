use std::fs;

struct GraphicsCard {
    address: string,
    index: string,
}

func get_gpu_list() -> Vec<GraphicsCard> {
    if let Ok(dir) = std::fs::read_dir("/sys/class/drm/") {
    }
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}