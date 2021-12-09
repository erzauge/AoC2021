use std::fs;
use std::collections::HashSet;

pub fn solve(){
    let input = fs::read_to_string("data/day9.dat").expect("file day9 fehlt");
    let map: Vec<Vec<u8>> =input.lines().map(|x|x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect()).collect();
    let lx=map.len();
    let ly=map[0].len();
    let mut loc_min:Vec<(usize,usize)>=vec!();
    let empty = vec!();
    let mut risk=0;
    for i in 0..lx{
        for j in 0..ly {
            let a = std::cmp::min(map.get((i as i32 -1) as usize).unwrap_or(&empty).get(j).unwrap_or(&255), map.get(i+1).unwrap_or(&empty).get(j).unwrap_or(&255));
            let b = std::cmp::min(map.get(i).unwrap_or(&empty).get((j as i32 -1) as usize).unwrap_or(&255), map.get(i).unwrap_or(&empty).get(j+1).unwrap_or(&255));
            let c = map[i][j];
            if *a>c&&*b>c {
                loc_min.push((i as usize,j as usize));
                risk+=c as u32+1;
            } 
        }
    }

    
    println!("risk sum {}",risk);
    let mut baisens =vec!();
    for i in loc_min{
        let mut set = HashSet::new();
        let mut buffer =vec![i];
        while let Some((x,y)) = buffer.pop() {
            if (*map.get((x as i32 -1) as usize).unwrap_or(&empty).get(y).unwrap_or(&9)<9){
                if (!set.contains(&(((x as i32 -1) as usize),y))) {
                    buffer.push((((x as i32 -1) as usize),y));
                }
            }
            if (*map.get(x+1).unwrap_or(&empty).get(y).unwrap_or(&9)<9){
                if (!set.contains(&(x+1,y))) {
                    buffer.push((x+1,y));
                }
            }
            if (*map.get(x).unwrap_or(&empty).get((y as i32 -1) as usize).unwrap_or(&9)<9){
                if (!set.contains(&(x,(y as i32 -1) as usize))) {
                    buffer.push((x,((y as i32 -1) as usize)));
                }
            }
            if (*map.get(x).unwrap_or(&empty).get(y+1).unwrap_or(&9)<9){
                if (!set.contains(&(x,y+1))) {
                    buffer.push((x,y+1));
                }
            }
            set.insert((x,y));
        }
        baisens.push(set);
    }

    let mut baisen_size:Vec<u32> = baisens.iter().map(|x|x.len() as u32).collect();
    baisen_size.sort();
    baisen_size.reverse();
    println!("lagest thre baisend product {}",baisen_size.iter().take(3).product::<u32>());
}