use std::{env, thread, time, vec};
use rand::Rng;
use ncurses::{ll::addstr, *};

fn mapchar(v: i32) -> char {
    return match v {
         1 => '█',
         0 => ' ',
        -1 => '~', // Out of bounds
        _ => '?'
    }
}

// scons window id
static mut ww: i8 = -1;

fn getchar(map: & Vec<Vec<i32>>, pos: (i32, i32)) -> char {
    if pos.0 < 0 || pos.1 < 0 {
        mapchar(-1)
    } else if pos.0 >= map.len() as i32 || pos.1 >= map[0].len() as i32 {
        mapchar(-1)
    } else {
        let ch = map[pos.0 as usize][pos.1 as usize];
        mapchar(ch)
    }
}

fn printmap(map: & Vec<Vec<i32>>, add: (i32, i32)){
    let x_size = map.len();
    let y_size = map[0].len();
    
    let mut lww = unsafe { // assustador
        ww
    };
    let screen_max = (getmaxx(&mut lww), getmaxy(&mut lww));

    for xi in 0 .. screen_max.0 {
        for yi in 1 .. screen_max.1 {
            let cx = xi + add.0;
            let cy = yi + add.1;
        }
    }
}

fn heatmap(map: & Vec<Vec<i32>>){
    /*let X = map.len();
    let Y = map[0].len();
    for x in 0..X {
        for y in 0..Y {
            let v = adjacencies(x, y, map);
            print!("{v} ");
        }
        println!();
    }*/
}

fn adjacencies(x: usize, y: usize, map: & Vec<Vec<i32>>) -> i32 {
    let X = map.len();
    let Y = map[0].len();

    let x_less: usize = if x == 0 { X - 1 } else { x - 1 };
    let x_more: usize = (x + 1) % X;
    let y_less: usize = if y == 0 { Y - 1 } else { y - 1 };
    let y_more: usize = (y + 1) % Y;
    map[x][y_less] + map[x][y_more] + map[x_less][y] + map[x_more][y]
    + map[x_less][y_less] + map [x_less][y_more] + map[x_more][y_less] + map[x_more][y_more]
}

fn nextmap(current: & Vec<Vec<i32>>, next: &mut Vec<Vec<i32>>){
    let X = current.len();
    let Y = current[0].len();

    for x in 0..current.len() {
        let row = current.get(x).expect("wuh?");
        for y in 0..Y {
            let adjacencies: i32 = adjacencies(x, y, current);
            let c_cell: i32 = current[x][y];
            if c_cell == 1 {
                if adjacencies < 2 || adjacencies > 3 {
                    next[x][y] = 0;
                } else {
                    next[x][y] = 1;
                }
            } else if c_cell == 0 {
                if adjacencies == 3 {
                    next[x][y] = 1;
                } else {
                    next[x][y] = 0;
                }
            }
        }
    }
}

fn resize(bmp: &mut Vec<Vec<i32>>, x: usize, y: usize){
    let lx = bmp.len();
    for _xi in lx..x {
        bmp.push(vec!());
    }
    for xi in 0..x{
        let row = bmp.get_mut(xi).expect("Error getting row {xi}");
        let ly = row.len();
        for _yi in ly..y {
            row.push(0);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Olá mundo :)");
    let mut bmp = [&mut vec!(),&mut vec!()];
    let n = 16;
    let mut size_x = 32;
    let mut size_y = 32;

    unsafe { ww = 2; }; // tenebroso

    resize(bmp.get_mut(0).unwrap(), size_x, size_x);
    resize(bmp.get_mut(1).unwrap(), size_y, size_y);
    
    let a = bmp.get_mut(0).unwrap();
    for x in 0..size_x {
            for y in 0..size_y {
            a[x][y] = rand::thread_rng().gen_range(0..n) / (n-1);
        }
    }

    let hmp: bool = args.contains(&String::from("--heatmap"));

    let mut i: usize = 0;
    loop {
        let next = (i + 1) % 2;
        let h = bmp.split_at_mut(1);
        let t = if i == 0 {(h.0,h.1)} else {(h.1,h.0)};
        
        nextmap(
            t.0.get_mut(0).expect("rip t0"),
            t.1.get_mut(0).expect("rip t1")
        );
        println!("{i} -> {next}");
        
        if hmp {heatmap(& bmp[i]);}
        printmap(& bmp[i],(0,0));
        i = next;
        thread::sleep(time::Duration::from_millis(125));
    }
}
