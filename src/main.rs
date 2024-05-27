use std::{thread::sleep, time::Duration};
use std::{collections::{HashMap, HashSet}, io::Write};

use autopilot::geometry::Point;
use enigo::{
    Button, Coordinate,
    Direction::Click,
    Enigo, Key, Keyboard, Mouse, Settings,
};
use serde::de;

mod wordlist;
mod config;

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let args = std::env::args().collect::<Vec<_>>();

    let contents = if args.len() > 1 {
        println!("Die Wordlist wird aud der Datei {} geladen", &args[1]);
        wordlist::load_wordlist(&args[1])
    } else {
        println!("Keine Datei angegeben. Standardwörterliste(deutsch) wird geladen");
        wordlist::load_wordlist("german.txt")
    };
    println!("Es wurden {} Wörter mit 5 Buchstaben geladen", contents.len());

    let config_data = if args.len() > 2 {
        println!("Die Konfigurationsdatei {} wird geladen", &args[2]);
        config::Config::load_from_file(&args[2])
    } else {
        println!("Keine Konfigurationsdatei angegeben. Standardkonfiguration wird geladen");
        config::Config::load_from_file("config-desktop.txt")
    };
    println!("Konfiguration wurde geladen");
    
    sleep(Duration::from_secs(1));
    enigo.move_mouse(config_data.initial_click_x, config_data.initial_click_y, Coordinate::Abs).unwrap();
    
    loop {
        println!("====================================");
        solve_extern(contents.clone(), &config_data);
        sleep(Duration::from_secs(3));
        enigo.move_mouse(config_data.restart_x, config_data.restart_y, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
        sleep(Duration::from_millis(800));
    }
}

fn solve_extern(mut contents: Vec<Vec<char>>, config_data: &config::Config) {
    let start = std::time::Instant::now();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let top = (config_data.top_left_square_x, config_data.top_left_square_y);
    let size = config_data.square_size;
    enigo.move_mouse(top.0 as i32, top.1 as i32, Coordinate::Abs).unwrap();
    enigo.button(Button::Left, Click).unwrap();

    sort_words(&mut contents);

    let mut row = 0;
    while row < 6 {
        let mut index = 0;
        let mut info = String::new();
        for (indx, word) in contents.iter().enumerate() {
            println!("{} mögliche Lösungen", contents.len() - indx);
            print!("Eingabe: {}", word.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
            std::io::stdout().flush().unwrap();
            for c in word {
                enigo.key(Key::Unicode(*c), Click).unwrap();
            }
            enigo.key(Key::Return, Click).unwrap();
            sleep(Duration::from_millis(470));
            let pre_result = read_row(row, top, size);
            if pre_result[0] == 0 {
                println!(" (nicht in wordlist)");
                if indx == contents.len() - 1 {
                    no_solution(row, &mut enigo);
                    return;
                }
                reset(&mut enigo);
                continue;
            }
            sleep(Duration::from_millis(1200));
            let result = read_row(row, top, size);
            println!(" {:?}", result);
            if result.iter().find(|&&e| e == 0).is_some() {
                if indx == contents.len() - 1 {
                    no_solution(row, &mut enigo);
                    return;
                }
                reset(&mut enigo);
                continue;
            }
            if result.iter().all(|&e| e == 3) {
                println!("Lösung gefunden: {} (time: {:.2}s)", word.iter().fold(String::new(), |mut a, c| {a.push(*c); a}), start.elapsed().as_secs_f32());
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
            (char_prob[i].get(&c).unwrap_or(&0.0) * 1000000.0) as i32 / word.iter().filter(|e| *e == c).count() as i32
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
        .map(|pixel| match pixel[0] {
            40..=79 => 1,
            100..=220 => 2,
            80..=99 => 3,
            _ => 0,
        })
        .collect()
}