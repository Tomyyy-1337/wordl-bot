#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub initial_click_pos: (i32, i32),
    pub top_left_square: (usize, usize),
    pub square_size: usize,
    pub restart_button: (i32, i32),
}

impl Config {
    pub fn load_from_file(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).unwrap();
        let data = contents.lines().map(|line| {
            let split = line.split(":");
            let value = split.last().unwrap().trim();
            value
        }).collect::<Vec<_>>();
        let mut tmp = data[0].split(",");
        let initial_click_pos = (tmp.next().unwrap().trim().parse().unwrap(), tmp.next().unwrap().trim().parse().unwrap());

        tmp = data[1].split(",");
        let top_left_square = (tmp.next().unwrap().trim().parse().unwrap(), tmp.next().unwrap().trim().parse().unwrap());

        let square_size = data[2].parse().unwrap();

        tmp = data[3].split(",");
        let restart_button = (tmp.next().unwrap().trim().parse().unwrap(), tmp.next().unwrap().trim().parse().unwrap());

        Config {
            initial_click_pos,
            top_left_square,
            square_size,
            restart_button,
        }
    }
}