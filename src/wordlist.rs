use std::{collections::HashSet, io::Write};

pub fn load_wordlist(file: &str) -> Vec<Vec<char>> {
    let mut contents: Vec<_> = std::fs::read_to_string(file).unwrap()
        .lines()
        .map(|word| word.to_lowercase().chars().collect::<Vec<_>>()) 
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
        
    let size = contents.len();
    contents = contents.into_iter()
        .filter(|word| word.len() == 5)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    if size != contents.len() {
        println!("Die Wörterliste wurde auf {} Wörter gefiltert", contents.len());
        print!("Änderungen speichern? [y/n]: ");
        
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() == "y" {
            print!("Dateiname: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            contents.sort_unstable();
            std::fs::write(
                input.trim(), 
                contents.iter()
                    .map(| v | 
                        v.iter().fold(String::new(), |mut a, c| {a.push(*c); a})
                    )
                    .collect::<Vec<_>>()
                    .join("\n")
                ).unwrap();
            println!("Datei gespeichert. Name: {}", input.trim());
        }
    }
    contents
}