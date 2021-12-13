
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("data/day13.dat").expect("file day13 fehlt");
    let mut iter =input.split("\n\n");
    let mut points:Vec<(u32,u32)> = iter.next().unwrap().lines().map(|a|{let mut a=a.split(","); (a.next().unwrap().parse::<u32>().unwrap(),a.next().unwrap().parse::<u32>().unwrap()) }).collect();
    let fold = iter.next().unwrap().lines();
    for f in fold {
        let mut f = f.split('=');
        let dir= f.next().unwrap();
        let pos =f.next().unwrap().parse::<u32>().unwrap();
        if dir=="fold along x"{
            points.iter_mut().for_each(|x|if x.0>pos {x.0=pos-(x.0-pos)});
        }
        else if dir=="fold along y" {
            points.iter_mut().for_each(|x|if x.1>pos {x.1=pos-(x.1-pos)});
        }
        points.sort();
        points.dedup();
        println!("{}",points.len());
    }
    let max_x =points.iter().map(|x|x.0).max().unwrap();
    let max_y =points.iter().map(|x|x.1).max().unwrap();
    println!("{} {}",max_x,max_y);

    for x in 0..=max_y{
        for y in 0..=max_x {
            if points.contains(&(y,x)){
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

