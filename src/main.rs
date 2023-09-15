use std::{env, io::ErrorKind};
use std::str::FromStr;
use async_std::io::{WriteExt, ReadExt};
use async_std::net::{TcpListener, TcpStream};

use bincode::{DefaultOptions, Options};
use serde::{Serialize, Deserialize};
mod constants;


use async_std::stream::StreamExt;
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



#[derive(Clone, Debug, Serialize, Deserialize)]

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

async fn start_game_conn(role:&Role, opp_port:i32, host_port:i32, opp_ip:String) -> Result<TcpStream, std::io::Error> {
    match role {

        Role::Server =>{

            let listener = TcpListener::bind(["0.0.0.0", host_port.to_string().as_str()].join(":").as_str()).await?;
            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next().await {
                let stream = stream?;
                println!("Success");
                return Ok(stream);
            }
            return Err(std::io::Error::new(ErrorKind::Other, "TCP Server error"));
        },
        Role::Client => {
            let mut stream = TcpStream::connect([opp_ip.to_string(), opp_port.to_string()].join(":")).await?;
            println!("Success");
            return Ok(stream);
        }
    }
}

fn get_printing_cell(board:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], x:usize, y:usize) -> bool {
    return board[constants::BOARD_SIZE-1-x][y];
} 

fn print_board(board:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], known:&[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE]){
    print!("-");
    for i in 0..constants::BOARD_SIZE {
        print!("-");
    }
    print!("-\n");


    for y in (0..constants::BOARD_SIZE).rev() {
        print!("-");

        for x in 0..constants::BOARD_SIZE {
            if known[x][y]{
                if board[x][y] {
                    print!("x");
                } else {
                    print!("o");
                }
            } else {
                print!("?");
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
    let known_board = [[true; constants::BOARD_SIZE]; constants::BOARD_SIZE];

    loop {

        let mut coord:Coord;
        let mut orientation:Orientation;
        let mut line:String;
        let mut boats = 0;


        'boatloop: for boat_len in constants::BOAT_SIZES {

            print_board(&board, &known_board);
            
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

async fn game_loop(stream:TcpStream, host_board:[[bool;constants::BOARD_SIZE];constants::BOARD_SIZE], role:&Role) -> Result<(),std::io::Error> {
    let mut known_board:[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE] = [[false; constants::BOARD_SIZE]; constants::BOARD_SIZE];
    let mut opp_board:[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE] = [[false; constants::BOARD_SIZE]; constants::BOARD_SIZE];

    let mut opp_knows:[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE] = [[false; constants::BOARD_SIZE]; constants::BOARD_SIZE];

    println!("Your board looks like this:\n");
    print_board(&host_board, &[[true; constants::BOARD_SIZE]; constants::BOARD_SIZE]);

    println!("Your opponents' board looks like this:\n");
    print_board(&host_board, &known_board);

    let mut won:bool;

    match role {
        Role::Client => {
            loop {
                if answer_turn(host_board, &stream, &mut opp_knows).await? {
                    won=false;
                    break;
                }
                if take_turn(&mut known_board, &mut opp_board, &stream).await? {
                    won=true;
                    break;
                }
            }
        },
        Role::Server => {
            loop {
                if take_turn(&mut known_board, &mut opp_board, &stream).await? {
                    won=true;
                    break;
                }
                if answer_turn(host_board, &stream, &mut opp_knows).await? {
                    won=false;
                    break;
                }
            }
        }
    }

    if won {
        println!("Congratulations, you won!");
    } else {
        println!("Commiserations, you lost.");
    }
    
        
    return Ok(());

}

async fn take_turn(mut known_board:&mut[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], mut opp_board:&mut[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], mut stream:&TcpStream) -> Result<bool, std::io::Error> {
    let coord:Coord;
    
    loop{
        println!("Your opponents board looks like this:\n");
        print_board(&opp_board, &known_board);
        println!("Pick a coordinate on your opponents board [x,y]\n");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        line = line.trim().to_lowercase();

        match Coord::from_str(line.to_lowercase().trim()) {
            Ok(e) => {
                coord = e;
                break;
            },
            Err(e) => {println!("Invalid coordinate entered. Try again.\n")}
        }
    }


    stream.write_all(serde_json::to_string(&coord).unwrap().as_bytes()).await?;
    stream.flush();

    let mut buf = vec![0u8; 1024];

    let n = stream.read(&mut buf).await?;

    if buf[0] == 1 {
        println!("Hit!");
        opp_board[coord.x][coord.y] = true;
    } else {
        println!("Miss!");
        opp_board[coord.x][coord.y] = false;
    }


    known_board[coord.x][coord.y] = true;

    println!("Your opponents board now looks like this:\n");
    print_board(&opp_board, &known_board);

    let mut boat_squares:usize = 0;
    let total_boat_squares:usize = BOAT_SIZES.iter().sum();

    for x in 0..constants::BOARD_SIZE {
        for y in 0..constants::BOARD_SIZE {
            if opp_board[x][y] && known_board[x][y] {
                boat_squares = boat_squares + 1;
                if boat_squares == total_boat_squares {
                    return Ok(true);
                }
            }
        }
    }


    return Ok(false);
}

async fn answer_turn(mut own_board:[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE], mut stream:&TcpStream,  mut opp_knows:&mut[[bool;constants::BOARD_SIZE]; constants::BOARD_SIZE]) -> Result<bool,std::io::Error> {
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;

    let res:String = String::from_utf8_lossy(&mut buf).to_string();
    let res = res.trim_matches(char::from(0));


    let coord:Coord = serde_json::from_str(res).unwrap();

    let arr:&mut[u8] = &mut[0;1];

    if own_board[coord.x][coord.y] {
        arr[0] = 1;
        println!("Your opponent got a hit on [{}, {}]\n", coord.x, coord.y);
    } else {
        println!("Your opponent missed on [{}, {}]\n", coord.x, coord.y);
    }
    
    opp_knows[coord.x][coord.y] = true;

    stream.write_all(arr).await?;
    stream.flush();

    for x in 0..constants::BOARD_SIZE {
        for y in 0..constants::BOARD_SIZE {
            if own_board[x][y] && !opp_knows[x][y] {
                return Ok(false);
            }
        }
    }

    return Ok(true);

}


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("Incorrect number of arguments\nUsage: cargo run <opponent_ip> <opponent_port> <host_role> <host_port>")
    }

    let ip = args[1].trim();

    let opp_port:i32 = args[2].parse().unwrap();

    let role:Role = match args[3].to_lowercase().trim() {
        "client" => {Role::Client},
        "server" => {Role::Server},
        _ => {
            panic!("Invalid role {} supplied\n", args[2])
        }
    };

    let host_port:i32 = args[4].parse().unwrap();

    let host_board:[[bool;constants::BOARD_SIZE];constants::BOARD_SIZE] = choose_board();

    
    let stream:TcpStream = start_game_conn(&role, opp_port, host_port, ip.to_string()).await.unwrap();


    game_loop(stream, host_board, &role).await;



}


