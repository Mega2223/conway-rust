use std::{process::exit, thread, time};
use rand::Rng;

const X: usize = 32;
const Y: usize = 32;

fn mapchar(v: i32) -> char {
    return match v {
        1 => '█',
        0 => ' ',
        _ => '?'
    }
}

fn printmap(map: [[i32; X]; Y]){
    print!("+ ");
    for x in map {
        for y in x {
            let c = mapchar(y);
            print!("{c}{c}");
        }
        println!();
    }
}

fn heatmap(map: [[i32; X]; Y]){
    for x in 0..X {
        for y in 0..Y {
            let v = adjacencies(x, y, map);
            print!("{v} ");
        }
        println!();
    }
}

fn adjacencies(x: usize, y: usize, map: [[i32; X]; Y]) -> i32 {
    let x_less: usize = if x == 0 { X - 1 } else { x - 1 };
    let x_more: usize = (x + 1) % X;
    let y_less: usize = if y == 0 { Y - 1 } else { y - 1 };
    let y_more: usize = (y + 1) % Y;
    map[x][y_less] + map[x][y_more] + map[x_less][y] + map[x_more][y]
    + map[x_less][y_less] + map [x_less][y_more] + map[x_more][y_less] + map[x_more][y_more]
}

fn nextmap(current: [[i32; X];  Y], next: &mut [[i32; X]; Y]){
    for x in 0..X {
        for y in 0..Y {
            let adjacencies = adjacencies(x, y, current);
            let c_cell = current[x][y];
            next[x][y] = if c_cell == 0 && adjacencies >= 3 {
                1
            } else if c_cell == 0 {
                0
            } else if adjacencies == 2 || adjacencies == 3 {
                1
            } else {
                0
            };
        }
    }
}

fn main() {
    println!("Olá mundo :)");
    let mut bmp: [[[i32; X]; Y]; 2] = [[[0; X]; Y]; 2];
    
    let n = 16;

    for x in 0..X {
            for y in 0..Y {
            bmp[0][x][y] = rand::thread_rng().gen_range(0..n) / (n-1);
        }
    }

    /*
    bmp[0][5][5] = 1;
    bmp[0][5][6] = 1;
    bmp[0][6][5] = 1;
    bmp[0][6][6] = 1;
    bmp[0][6][7] = 1;
    */

    let hmp = false;

    let mut i: usize = 0;
    loop {
        let next = (i + 1) % 2;
        nextmap(bmp[i],&mut bmp[next]);
        println!("{i} -> {next}");
        printmap(bmp[i]);
        //printmap(bmp[next]);
        if hmp {heatmap(bmp[i]);}
        i = next;
        thread::sleep(time::Duration::from_millis(250));
    }
}
