use std::{env, thread, time, vec};
use rand::Rng;

fn mapchar(v: i32) -> char {
    return match v {
        1 => '█',
        0 => ' ',
        _ => '?'
    }
}

fn printmap(map: & Vec<Vec<i32>>){
    print!("\n    ");
    let X = map.len();
    let Y = map[0].len();
    for _i in 0..Y-1 {
        let __i = _i % 10;
        print!("{__i} ");
    }
    print!("\n  + ");
    for _i in 0..Y-1 {
        print!("- ");
    }
    println!("+");
    for ix in 1..X {
        let ixx = ix % 10;
        print!("{ixx} + ");
        for iy in 1..Y {
            let c = mapchar(map[ix][iy]);
            print!("{c}{c}");
        }
        println!("+");
    }
    /*for x in map {
        print!("+ ");
        for y in x {
            let c = mapchar(y);
            print!("{c}{c}");
        }
        println!("+");
    }*/
    print!("  + ");
    for _i in 0..Y-1 {
        print!("- ");
    }
    println!("+");
}

fn heatmap(map: & Vec<Vec<i32>>){
    let X = map.len();
    let Y = map[0].len();
    for x in 0..X {
        for y in 0..Y {
            let v = adjacencies(x, y, map);
            print!("{v} ");
        }
        println!();
    }
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
        
        //let ue = t.0.get_mut(0).expect("f");
        nextmap(
            t.0.get_mut(0).expect("rip t0"),
            t.1.get_mut(0).expect("rip t1")
        );
        println!("{i} -> {next}");
        
        if hmp {heatmap(& bmp[i]);}
        printmap(& bmp[i]);
        i = next;
        thread::sleep(time::Duration::from_millis(125));
    }
}
