use std::env;
use std::str::FromStr;


mod constants;


use constants::BOAT_SIZES;
use regex::Regex;

enum Role {
    Client, 
    Server
}

enum Orientation {
    Up,
    Down,
    Left,
    Right
}

impl FromStr for Orientation {
    type Err = ();
    fn from_str(input: &str) -> Result<Orientation, Self::Err>{
        match input {
            "left" => Ok(Orientation::Left),
            "right" => Ok(Orientation::Right),
            "up" => Ok(Orientation::Up),
            "down" => Ok(Orientation::Down),
            _ => Err(())
        }
    }
}



struct Coord {
    x: usize, 
    y: usize
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(input: &str) -> Result<Coord, Self::Err> {

        let coord_re:Regex = Regex::new("^\\[(?P<x>[0-9]),(?P<y>[0-9])\\]$").unwrap();


        
        if coord_re.is_match(input) {
            let Some(captures) = coord_re.captures(input) else {return Err(())};
        

            return Ok(Coord{x:captures["x"].parse().unwrap(),y:captures["y"].parse().unwrap()});
        } else {
            return Err(());
        }

    }
}

fn get_board_number(board:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], x:usize, y:usize) {
    return;
}

fn start_game_conn(role:Role) {
    return;
}

fn get_printing_cell(board:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], x:usize, y:usize) -> bool {
    return board[constants::BOARD_SIZE-1-x][y];
} 

fn print_board(board:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE]){
    print!("-");
    for i in 0..constants::BOARD_SIZE {
        print!("-");
    }
    print!("-\n");


    for y in (0..constants::BOARD_SIZE).rev() {
        print!("-");

        for x in 0..constants::BOARD_SIZE {
            if board[x][y] {
                print!("x");
            } else {
                print!("o");
            }
        }

        print!("-\n");
    }

    print!("-");
    for i in 0..constants::BOARD_SIZE {
        print!("-");
    }
    print!("-\n");
}


fn place_boat(boat_size:usize, orientation:Orientation, board:&mut [[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], start_coord:Coord) -> bool {


    if start_coord.x < 0 || start_coord.y < 0 || start_coord.x >= constants::BOARD_SIZE || start_coord.y >= constants::BOARD_SIZE {
        return false;
    }

    match orientation {
        Orientation::Up => {if (start_coord.y + boat_size) > constants::BOARD_SIZE {return false}},
        Orientation::Down => {if start_coord.y < (boat_size - 1) {return false}},
        Orientation::Left => {if start_coord.x < (boat_size - 1) {return false}},
        Orientation::Right => {if (start_coord.x + boat_size) > constants::BOARD_SIZE {return false}},
    }

    let check_coord:Coord = start_coord;

    for i in 0..boat_size {
        match orientation {
            Orientation::Up => {        
                if board[check_coord.x][check_coord.y + i] {
                    return false;
                }
            },
            Orientation::Down => {        
                if board[check_coord.x][check_coord.y - 1] {
                    return false;
                }
            },
            Orientation::Left => {        
                if board[check_coord.x - i][check_coord.y] {
                    return false;
                }
            },
            Orientation::Right => {        
                if board[check_coord.x + i][check_coord.y] {
                    return false;
                }
            }
        }
    }

    for i in 0..boat_size {
        match orientation {
            Orientation::Up => {        
                board[check_coord.x][check_coord.y + i] = true;
            },
            Orientation::Down => {        
                board[check_coord.x][check_coord.y - i] = true;
            },
            Orientation::Left => {        
                board[check_coord.x - i][check_coord.y] = true;
            },
            Orientation::Right => {        
                board[check_coord.x + i][check_coord.y] = true;
            }
        }
    }

    return true;
}

fn choose_board() -> [[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE] {

    let mut board:[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE] = [[false; constants::BOARD_SIZE]; constants::BOARD_SIZE];

    loop {

        let mut coord:Coord;
        let mut orientation:Orientation;
        let mut line:String;
        let mut boats = 0;


        'boatloop: for boat_len in constants::BOAT_SIZES {

            print_board(&board);
            
            println!("The next boat is {} squares long.\n", boat_len);
           

            loop {
                loop{
                    println!("Enter its starting square [x,y]\n");
                    line = String::new();
                    std::io::stdin().read_line(&mut line);
                    line = line.trim().to_lowercase();

                    if line.eq("restart") {
                        println!("Resetting board");
                        break 'boatloop;
                    }

                    match Coord::from_str(line.to_lowercase().trim()) {
                        Ok(e) => {
                            coord = e;
                            break;
                        },
                        Err(e) => {println!("Invalid coordinate entered. Try again.\n")}
                    }
                }

                

                loop {
                    println!("Enter its orientation");

                    line = String::new();
                    std::io::stdin().read_line(&mut line);
                    line = line.trim().to_lowercase();
                
                    if line.eq("restart") {
                        println!("Resetting board");
                        break 'boatloop;
                    }

                    match Orientation::from_str(line.as_str()){
                        Ok(e) => {orientation = e;break;},
                        Err(e) => {println!("Invalid coordinate entered. Try again.\n")} 
                    }
                }

                if (!place_boat(boat_len, orientation, &mut board, coord)) {
                    println!("Invalid boat placement. Try again.");
                } else {
                    boats = boats + 1;
                    if boats == BOAT_SIZES.len() {
                        return board;
                    }
                    break;
                }
            }
           
        }


    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("Incorrect number of arguments\nUsage: cargo run <opponent_ip> <host_port> <host_role>")
    }

    let ip = args[1].trim();

    let port: &str = args[2].trim();

    let role:Role = match args[3].to_lowercase().trim() {
        "client" => {Role::Client},
        "server" => {Role::Server},
        _ => {
            panic!("Invalid role {} supplied\n", args[2])
        }
    };

    let host_board:[[bool;constants::BOARD_SIZE];constants::BOARD_SIZE] = choose_board();


}


