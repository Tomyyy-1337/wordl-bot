use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub initial_click_x: i32,
    pub initial_click_y: i32,
    pub top_left_square_x: usize,
    pub top_left_square_y: usize,
    pub square_size: usize,
    pub restart_x: i32,
    pub restart_y: i32
}

impl Config {
    pub fn load_from_file(file: &str) -> Self {
        let data = std::fs::read_to_string(file).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}