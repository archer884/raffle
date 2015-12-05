extern crate rand;

use std::iter::FromIterator;
use std::collections::HashSet;
use std::fs::{ File, OpenOptions };
use std::io::{ BufRead, BufReader, Write };

use rand::{ Rng, OsRng };

fn main() {
    match read_paths() {
        Err(_) => panic!("<entrant> and <winner> paths both required"),
        Ok((entrants_path, winners_path)) => {
            let entrants: Vec<_> = read_into(&entrants_path);
            let winners: HashSet<_> = read_into(&winners_path);

            let eligible: Vec<_> = entrants.iter().filter(
                |entrant| !winners.contains(*entrant)
            ).collect();

            let winner = match OsRng::new() {
                Err(_) => rand::thread_rng().choose(&eligible),
                Ok(mut rng) => rng.choose(&eligible),
            };

            match winner {
                None => println!("no winner selected"),
                Some(winner) => {
                    append_to_file(&winners_path, &winner);
                    println!("{}", winner);
                }
            }
        }
    }
}

fn read_into<T: Default + FromIterator<String>>(path: &str) -> T {
    match File::open(path).map(|f| BufReader::new(f)) {
        Err(_) => T::default(),
        Ok(reader) => reader.lines().filter_map(
            |line| line.ok().map(|line| line.trim().to_owned())
        ).collect()
    }
}

fn append_to_file(path: &str, content: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path);

    match file {
        Err(_) => panic!("unable to append to winner file"),
        Ok(mut file) => { write!(file, "{}\n", content).ok(); }
    }
}

fn read_paths() -> Result<(String, String), ()> {
    let paths: Vec<_> = std::env::args().skip(1).collect();
    if paths.len() == 2 {
        Ok((
            paths[0].to_owned(),
            paths[1].to_owned(),
        ))
    } else {
        Err(())
    }
}
