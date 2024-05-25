use std::{thread::sleep, time::Duration};
use std::{collections::{HashMap, HashSet}, io::Write};


use autopilot::geometry::Point;
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let mut contents: Vec<_> = std::fs::read_to_string(&args[1]).unwrap()
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

    println!("Es wurden {} Wörter mit 5 Buchstaben geladen", contents.len());

    sleep(Duration::from_secs(1));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(670, 870, Coordinate::Abs).unwrap();
    
    loop {
        println!("================================================");
        solve_extern(contents.clone());
        sleep(Duration::from_secs(3));
        enigo.move_mouse(670, 870, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
        sleep(Duration::from_secs(1));
    }
}

fn solve_extern(mut contents: Vec<Vec<char>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    // enigo.button(Button::Left, Press)?;
    // enigo.button(Button::Left, Release)?;
    
    let top = (675,464);
    let size = 68;
    enigo.move_mouse(top.0 as i32, top.1 as i32, Coordinate::Abs).unwrap();
    enigo.button(Button::Left, Click).unwrap();

    sort_words(&mut contents);

    let mut row = 0;
    while row < 6 {
        let mut index = 0;
        let mut info = String::new();
        for (indx, word) in contents.iter().enumerate() {
            println!("{} mögliche Lösungen", contents.len() - indx);
            println!("Eingabe: {}", word.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
            for c in word {
                enigo.key(Key::Unicode(*c), Click).unwrap();
            }
            enigo.key(Key::Return, Click).unwrap();
            sleep(Duration::from_secs(2));
            let result = read_row(row, top, size);
            println!("Bewertung: {:?}", result);
            if result.iter().find(|&&e| e == 0).is_some() {
                if indx == contents.len() - 1 {
                    no_solution(row, &mut enigo);
                    return;
                }
                reset(&mut enigo);
                continue;
            }
            if result.iter().all(|&e| e == 3) {
                println!("Lösung gefunden: {}", word.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
                return;
            }
            info = result.iter().fold(String::new(), |mut a, c| {a.push_str(&c.to_string()); a});
            row += 1;
            index += indx;
            break;
        }
        contents = solve(contents, index, &info);
        if contents.len() == 0 {
            no_solution(row, &mut enigo);
            return;
        }
    }
}

fn no_solution(row: usize, enigo: &mut Enigo) {
    reset(enigo);
    println!("Keine Lösung gefunden.");
    let word = "bares";
    for _ in row..6 {
        for c in word.chars() {
            enigo.key(Key::Unicode(c), Click).unwrap();
        }
        enigo.key(Key::Return, Click).unwrap();
        sleep(Duration::from_secs(2));
    }
}

fn reset(enigo: &mut Enigo) {
    for _ in 0..5 {
        enigo.key(Key::Backspace, Click).unwrap();
    }
    sleep(Duration::from_millis(10));
}

fn sort_words(contents: &mut Vec<Vec<char>>) {
    let chars: Vec<char> = contents.iter().flatten().collect::<HashSet<_>>().into_iter().cloned().collect();
    let char_prob: Vec<HashMap<char, f32>> = (0..5).map(|i| {
        chars.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(*c, contents.iter().filter(|w| w[i] == *c).count() as f32 / contents.len() as f32);
            map
        }) 
    }).collect();

    contents.sort_by_cached_key(|word| {
        -word.iter().enumerate().map(|(i, c)| 
            (char_prob[i].get(&c).unwrap_or(&0.0) * 100000.0) as i32 / word.iter().filter(|e| *e == c).count() as i32
        ).sum::<i32>()
    });
}

fn solve(mut contents: Vec<Vec<char>>, index: usize, result: &str) -> Vec<Vec<char>> {
    let best = contents[index].clone();
    sort_words(&mut contents);
    
    let mut input = result.trim().chars().take(5).collect::<Vec<_>>();
    input = input.iter().enumerate()
        .map(|(j,c)| match c {
            '1' if input.iter().enumerate().any(|(i,c)| *c != '1' && best[j] == best[i]) => '2',
            c => *c,
        })
        .collect();

    for (i,(c, b)) in input.iter().zip(best.into_iter()).enumerate() {
        match c {
            '1' => contents.retain(|w| !w.contains(&b)),
            '2' => contents.retain(|w| w.contains(&b) && w[i] != b),
            '3' => contents.retain(|w| w[i] == b),
            _ => println!("Ungültige Eingabe. Verhalten nicht definiert"),
        } 
    }

    contents
}

fn read_row(indx: usize, top: (usize, usize), size: usize) -> Vec<u8> {
    (0..5).map(|j| (top.0 + j * size, top.1 + indx * size))
        .map(|(x, y)| autopilot::screen::get_color(Point::new(x as f64, y as f64)).unwrap())
        .map(|pixel| match (pixel[0], pixel[1], pixel[2], pixel[3]) {
            (58, 58, 60, 255) => 1,
            (163, 135, 9, 255) => 2,
            (83, 141, 78, 255) => 3,
            _ => 0,
        })
        .collect()
}