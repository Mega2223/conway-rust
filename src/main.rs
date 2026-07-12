use std::{thread, time};
use rand::Rng;

const X: usize = 32;
const Y: usize = 32;

fn printmap(map: [[i32; X]; Y]){
    for x in map {
        for y in x {
            print!("{y} ");
        }
        println!();
    }
}

fn adjacencies(x: usize, y: usize, map: [[i32; X]; Y]) -> i32 {
    let x_less: usize = if x == 0 { X - 1 } else { x - 1 };
    let x_more: usize = (x + 1) % X;
    let y_less: usize = if y == 0 { Y - 1 } else { y - 1 };
    let y_more: usize = (y + 1) % Y;
    
    map[y][x_less] + map[y][x_more] + map[y_less][x] + map[y_more][x] +
    map[y_less][x_less] + map [y_less][x_more] + map[y_more][x_less] + map[y_more][x_more]
}

fn nextmap(current: [[i32; X];  Y], next: &mut [[i32; X]; Y]){
    for i in 0..X {
        for j in 0..Y {
            let adjacencies = adjacencies(i, j, current);
            if adjacencies == 2 || adjacencies == 3 {
                next[i][j] = 1;
            } else {
                next[i][j] = 0;
            }
            ;;let v = next[i][j];
            // println!("[{i}][{j}] = {v}");
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut bmp: [[[i32; X]; Y]; 2] = [[[0; X]; Y]; 2];

    for x in 0..X {
        for y in 0..Y {
            bmp[0][x][y] = rand::thread_rng().gen_range(0..2);
        }
    }

    let mut i: usize = 0;
    loop {
        let next = (i + 1) % 2;
        nextmap(bmp[i],&mut bmp[next]);
        println!("{i} -> {next}");
        printmap(bmp[i]);
        i = next;
        thread::sleep(time::Duration::from_millis(100));
    }
}
