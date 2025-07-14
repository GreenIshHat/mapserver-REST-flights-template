use std::{fmt, thread};
use std::time::Duration;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::net::{Tcplistener, TcpStream};

use rand::Rng;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 10;
const MAX_NUM_AIRCRAFTS: i32 = 10;
const MIN_NUM_AIRCRAFTS: i32 = 10;

#[derive(Clone, Debug)]
enum Direction {
    N, NE, E, SE, S, SW, W, NW
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::N  => write!(f, "↑ "),
            Direction::NE => write!(f, "↗ "),
            Direction::E  => write!(f, "→ "),
            Direction::SE => write!(f, "↘︎ "),
            Direction::S  => write!(f, "↓ "),
            Direction::SW => write!(f, "↙ "),
            Direction::W  => write!(f, "← "),
            Direction::NW => write!(f, "↖︎ "),
        }
    }
}

#[derive(Clone, Debug)]
struct Flight {
    id: String,
    x: i32,
    y: i32,
    direction: Direction,
}

fn main() {

    let mut traffic_data: Vec<Flight> = Vec::new();

    generate_map(&mut traffic_data);
    dbg!(&traffic_data);
    draw_char_map(&traffic_data);

    // periodically move the aircrafts
    let handle = thread::spawn(move || {

        let mut skip_counter = 0;
        loop {

            if skip_counter == 3 {
              move_aircrafts(&mut traffic_data);
              // draw_char_map(&traffic_data);
              skip_counter = 0;
            } else {
                skip_counter += 1;
            }

            sleep(Duration::from_millis(300));
        };

    });

    // other code to run...

    // REST API SERVER
     
    
    handle.join().unwrap();

}

fn add_new_flight(data_set: & mut Vec<Flight>) {

    let mut rng = rand::thread_rng();
    let letter1: char = rng.gen_range(b'A'..b'Z') as char;
    let letter2: char = rng.gen_range(b'A'..b'Z') as char;
    let number: u32 = rng.gen_range(10..9999);
    let new_id = format!("{}{}{:02}", letter1, letter2, number);

    // generate random x, y coordinates
    let new_x = rand::thread_rng().gen_range(0..MAP_WIDTH);
    let new_y = rand::thread_rng().gen_range(0..MAP_HEIGHT);

    // generate a random direction
    let dir = rand::thread_rng().gen_range(0..8);
    let new_dir = match dir {
        0 => Direction::N,
        1 => Direction::NE,
        2 => Direction::E,
        3 => Direction::SE,
        4 => Direction::S,
        5 => Direction::SW,
        6 => Direction::W,
        7 => Direction::NW,
        _ => Direction::N
    };

    data_set.push(Flight{id: new_id, x: new_x, y: new_y, direction: new_dir});

}

fn draw_char_map(data_set: &[Flight]) {

    let mut lock = stdout().lock();
    for y in 0..(MAP_HEIGHT) {
        write!(lock, " ").unwrap();
        for _ in 0..(MAP_WIDTH) {
            write!(lock, "-- ").unwrap();
        }
        write!(lock, "\r\n").unwrap();
        for x in 0..(MAP_WIDTH) {
            write!(lock, "|").unwrap();
            // is there an aircraft in this box's coordinates?
            let ufo = data_set
                .iter()
                .find(|flight| flight.x == x && flight.y == y);
            match ufo {
                None => write!(lock, "  ").unwrap(),
                Some(f) => write!(lock, "{}", f.direction.to_string()).unwrap()
            }
        }
        write!(lock, "|\r\n").unwrap();
    }
    // print the bottom line
    for _ in 0..(MAP_WIDTH) {
        write!(lock, " --").unwrap();
    }
    write!(lock, "\r\n").unwrap();

}

fn generate_map(data_set: &mut Vec<Flight>) {
    let num_aircrafts = rand::thread_rng()
        .gen_range(MIN_NUM_AIRCRAFTS..(MAX_NUM_AIRCRAFTS+1));
    for _ in 0..num_aircrafts {
        add_new_flight(data_set);
    }
}

fn move_aircrafts(data_set: &mut [Flight]) {

    for i in 0..data_set.iter().count() {
        match &data_set[i].direction {
            Direction::N => {
                data_set[i].y = data_set[i].y - 1;
                if data_set[i].y < 0 {
                    data_set[i].y = MAP_HEIGHT - 1;
                }
            }

            Direction::NE => {
                data_set[i].y = data_set[i].y - 1;
                if data_set[i].y < 0 {
                    data_set[i].y = MAP_HEIGHT - 1;
                }
                data_set[i].x = data_set[i].x + 1;
                if data_set[i].x >= MAP_WIDTH {
                    data_set[i].x = 0;
                }
            }

            Direction::E => {
                data_set[i].x = data_set[i].x + 1;
                if data_set[i].x >= MAP_WIDTH {
                    data_set[i].x = 0;
                }
            }

            Direction::SE => {
                data_set[i].x = data_set[i].x + 1;
                if data_set[i].x >= MAP_WIDTH {
                    data_set[i].x = 0;
                }
                data_set[i].y = data_set[i].y + 1;
                if data_set[i].y >= MAP_HEIGHT {
                    data_set[i].y = 0;
                }
            }

            Direction::S => {
                data_set[i].y = data_set[i].y + 1;
                if data_set[i].y >= MAP_HEIGHT {
                    data_set[i].y = 0;
                }
            }

            Direction::SW => {
                data_set[i].y = data_set[i].y + 1;
                if data_set[i].y >= MAP_HEIGHT {
                    data_set[i].y = 0;
                }
                data_set[i].x = data_set[i].x - 1;
                if data_set[i].x < 0 {
                    data_set[i].x = MAP_WIDTH - 1;
                }
            }

            Direction::W => {
                data_set[i].x = data_set[i].x - 1;
                if data_set[i].x < 0 {
                    data_set[i].x = MAP_WIDTH - 1;
                }
            }

            Direction::NW => {
                data_set[i].x = data_set[i].x - 1;
                if data_set[i].x < 0 {
                    data_set[i].x = MAP_WIDTH - 1;
                }
                data_set[i].y = data_set[i].y - 1;
                if data_set[i].y < 0 {
                    data_set[i].y = MAP_HEIGHT - 1;
                }
            }
            
        }
    }
}
