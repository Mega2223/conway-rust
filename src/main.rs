use std::{env, ops::Add, thread, time, vec};
use rand::Rng;
use pancurses::*;

fn mapchar(v: i32) -> char {
    return match v {
        1 => '█',
        0 => ' ',
        -1 => '~', // Out of bounds
        _ => '?'
    }
}

fn getchar(map: & Vec<Vec<i32>>, pos: (i32, i32), max_d: (i32, i32)) -> char {
    let mut pos = (pos.0, pos.1);
    pos.0 = pos.0.rem_euclid(max_d.0);
    pos.1 = pos.1.rem_euclid(max_d.1);
    if pos.0 < 0 || pos.1 < 0 {
        mapchar(-1)
    } else if pos.0 >= map.len() as i32 || pos.1 >= map[0].len() as i32 {
        mapchar(-1)
    } else {
        let ch = map[pos.0 as usize][pos.1 as usize];
        mapchar(ch)
    }
}

fn printmap(map: & Vec<Vec<i32>>, add: (i32, i32), window: & Window, heatmap: bool){
    let x_size = map.len() as i32;
    let y_size = map[0].len() as i32;
     
    let screen_max = (
        window.get_max_x(),
        window.get_max_y()
    );

    for xi in 0 .. screen_max.0 {
        for yi in 0 .. screen_max.1 {
            let cx = xi/2 + add.0;
            let cy = yi + add.1;
            window.mv(yi, xi);
            let mut ch = getchar(map, (cx,cy), (x_size, y_size));
            ch = if ch == ' ' && heatmap {
                let adj = adjacencies(cx as usize, cy as usize, map);
                if adj > 0 { adj.to_string().chars().next().unwrap_or('!') }
                else { ' ' }
            } else { ch };
            window.addnstr(ch.to_string() + ch.to_string().as_str(),3);
        }
    }
}

fn adjacencies(x: usize, y: usize, map: & Vec<Vec<i32>>) -> i32 {
    let sx = map.len();
    let sy = map[0].len();
    if x >= sx || y >= sy { return 0; }

    let x_less: usize = if x == 0 { sx - 1 } else { x - 1 };
    let x_more: usize = (x + 1) % sx;
    let y_less: usize = if y == 0 { sy - 1 } else { y - 1 };
    let y_more: usize = (y + 1) % sy;
    map[x][y_less] + map[x][y_more] + map[x_less][y] + map[x_more][y]
    + map[x_less][y_less] + map [x_less][y_more] + map[x_more][y_less] + map[x_more][y_more]
}

fn nextmap(current: & Vec<Vec<i32>>, next: &mut Vec<Vec<i32>>){
    for x in 0..current.len() {
        let row = current.get(x).expect("wuh?");
        for y in 0..row.len() {
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
    let n = 5;
    let size_x = 64; // TODO: Faz isso ser argumento da aplicaçãao
    let size_y = 64;
    let window = initscr();
    window.nodelay(true);
    noecho();
    pancurses::beep();
    pancurses::cbreak();

    resize(bmp.get_mut(0).unwrap(), size_x, size_x);
    resize(bmp.get_mut(1).unwrap(), size_y, size_y);

    let a = bmp.get_mut(0).unwrap();
    for x in 0..size_x {
            for y in 0..size_y {
            a[x][y] = rand::thread_rng().gen_range(0..n) / (n-1);
        }
    }

    let hmp: bool = args.contains(&String::from("--heatmap"));
    let mut pos = (0,0);

    let mut i: usize = 0;
    loop {
        let next = (i + 1) % 2;
        let h = bmp.split_at_mut(1);
        let t = if i == 0 {(h.0,h.1)} else {(h.1,h.0)};
        
        nextmap(
            t.0.get_mut(0).expect("rip t0"),
            t.1.get_mut(0).expect("rip t1")
        );
        // println!("{i} -> {next}");
        
        printmap(& bmp[i],pos, &window, hmp);
        i = next;
        
        window.mv(0,120);
        loop {
            let ch = match window.getch() {
                Some(Input::Character(c)) => c,
                None => {break;},
                _ => '?'
            };
            match ch {
                'W' | 'w' => pos.1 -= 1,
                'A' | 'a' => pos.0 -= 1,
                'S' | 's' => pos.1 += 1,
                'D' | 'd' => pos.0 += 1,
                _ => {}
            };
        };
        
        window.addstr(format!("  ({}  {})  ", pos.0, pos.1));
        window.refresh();

        thread::sleep(time::Duration::from_millis(50));
    }
}
