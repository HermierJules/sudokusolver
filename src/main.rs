use std::{io::{self, Write}, char::from_u32};
use bevy::{prelude::*,
            window::{PresentMode,WindowPlugin}};
//fn validate_grid(g : i128) -> bool 

struct Complete_grid {
    comp : i128,
    grid : [i128; 10],
}

impl Complete_grid {
    fn clone(&self) -> Complete_grid {
        Complete_grid {comp : self.comp,
        grid: self.grid.clone(),
    }
    }
    fn copy(&self) -> Complete_grid {
        Complete_grid {comp : self.comp,
            grid : self.grid.clone(),
        }
    }
}
fn popcount(g : i128) -> i128{
   let mut n = g;
    let mut sum = 0;
    while n != 0 {
        sum = sum + (n % 2);
        n = n / 2;
    }
    return sum;
}

fn print_grid(g : i128) -> (){
    let mut gr = g;
    for _ in 0..9 {
        let mut s = String::new();
        for _ in 0..9 {
            let b = gr % 2;
            if b == 1 {
                s.push('#');
            }
            else {
                s.push('.');
            }
            gr = gr / 2;
        }
        println!("{}",s);
    }
}



fn square_mask(n : i32)  -> i128 {
    let og_mask = 0b111000000111000000111;
    let x = n % 3;
    let y = n / 3;
    return og_mask << ((x * 3 + y * 3* 9));
}

fn line_mask(n: i32) -> i128 {
    let og_mask = 0b111111111;
    return og_mask << (n * 9);
}

fn column_mask(n: i32) -> i128 {
    let og_mask = 0b100000000100000000100000000100000000100000000100000000100000000100000000100000000100000000 >> 8;
    return og_mask << n;
}

fn check_partial_grid(g:i128) -> bool {
    for i in 0..9 {
        let n1 = popcount(g & square_mask(i));
        let n2 =popcount(g & line_mask(i));
        let n3 =popcount(g & column_mask(i));
        if n1 > 1 || n2 > 1 || n3 > 1 {return false;}
    }
    true
}

fn check_complete_grid(gr : &Complete_grid) -> bool {
    //assuming there is no overlap between grids
    check_partial_grid(gr.grid[0]) && check_partial_grid(gr.grid[1]) && check_partial_grid(gr.grid[2]) && check_partial_grid(gr.grid[3]) && check_partial_grid(gr.grid[4]) && check_partial_grid(gr.grid[5]) && check_partial_grid(gr.grid[6]) && check_partial_grid(gr.grid[7]) && check_partial_grid(gr.grid[8]) && check_partial_grid(gr.grid[9])  
}

fn next_case(gr: &Complete_grid) -> i64 {
    let mut i = 0;
    let mut n = gr.comp;
    while n % 2 != 0 {
        i+=1;
        n = n / 2;
    }
    return i;
}


fn play_move(gr: Complete_grid, case: i64, val: i64) -> Complete_grid {
    let mut new_gr = gr.clone(); // Clone the struct
    let add_mask: i128 = 0b1 << case;
    
    match val {
        0 => {
            new_gr.comp += add_mask;
            new_gr.grid[0] += add_mask;
        }
        1..=9 => {
            new_gr.comp += add_mask;
            new_gr.grid[val as usize] += add_mask;
        }
        _ => {}
    }
    
    new_gr // Return the modified copy of the struct
}


fn solve(gr: Complete_grid) -> Option<Complete_grid> {
    if !check_complete_grid(&gr) {
        return None;
    }
    let c = next_case(&gr);
    if popcount(gr.comp) == 81 {return Some(gr);}
    for i in 0..10 {
        //println!("currently on {} val {}", c, i);
        io::stdout().flush().unwrap();
        let g : Complete_grid = play_move(gr.clone(), c, i);
        if let Some(solution) = solve(g) {
            return Some(solution);
        }
    }
    return None;
}


fn to_array(gr : &Complete_grid) -> [usize;81] {
    let mut t = [0; 81];
    for i in 0..10 {
        let mut n = gr.grid[i];
        let mut k = 0;
        while n != 0 {
            if n % 2 == 1 {
                t[k] = i;
            }
            n = n / 2;
            k += 1;
        }
    }
    t
}

fn print_array(t : [usize;81]) -> () {
    for i in 0..9 {
        let mut s = String::new();
        for j in 0..9 {
            let digit = t[9 * i + j];
            let c = std::char::from_digit(digit as u32, 10).unwrap_or('.');
            s.push(c);
        }
        println!("{}", s);
    }
}


fn launch(mut commands : Commands, asset_server: Res<AssetServer>){
    let font = asset_server.load("roboto.ttf");

}

fn main() {
   // let gr : Complete_grid = Complete_grid {
    //    comp : 0b100101001101010101000101000101000101010000010101000101101010101100101001,
     //   grid : [0b0; 10],
    //};
    let mut gr = Complete_grid {
        comp : 0b0,
        grid : [0b0; 10]
    };
    gr = play_move(gr, 0, 3);
    gr = play_move(gr, 3, 8);
    gr = play_move(gr, 5, 1);
    gr = play_move(gr, 8, 2);
    gr = play_move(gr, 9, 2);
    gr = play_move(gr, 11, 1);
    gr = play_move(gr, 13, 3);
    gr = play_move(gr, 15, 6);
    gr = play_move(gr, 17, 4);
    gr = play_move(gr, 21, 2);
    gr = play_move(gr, 23, 4);
    gr = play_move(gr, 27, 8);
    gr = play_move(gr, 29, 9);
    gr = play_move(gr, 33, 1);
    gr = play_move(gr, 35, 6);
    gr = play_move(gr, 37, 6);
    gr = play_move(gr, 43, 5);
    gr = play_move(gr, 45, 7);
    gr = play_move(gr, 47, 2);
    gr = play_move(gr, 51, 4);
    gr = play_move(gr, 53, 9);
    gr = play_move(gr, 57, 5);
    gr = play_move(gr, 59, 9);
    gr = play_move(gr, 63, 9);
    gr = play_move(gr, 65, 4);
    gr = play_move(gr, 67, 8);
    gr = play_move(gr, 69, 7);
    gr = play_move(gr, 71, 5);
    gr = play_move(gr, 72, 6);
    gr = play_move(gr, 75, 1);
    gr = play_move(gr, 77, 7);
    gr = play_move(gr, 80, 3);
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        ))
            .add_systems(Startup, launch)
        .run();
    //gr = solve(gr).expect("lol");
   // let t = to_array(&gr);
   // print_array(t);
   // print_grid(gr.comp);
}
