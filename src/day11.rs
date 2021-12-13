use std::fs;

fn step(map:&mut Vec<Vec<(u32,bool)>>)->u32{
    map.iter_mut().for_each(|x|x.iter_mut().for_each(|y|{y.0+=1;y.1=true}));
    while map.iter().any(|x|x.iter().any(|y|y.0>9&&y.1))  {
        for (x,a) in map.clone().iter().enumerate() {
            for (y,_) in a.iter().enumerate().filter(|(_,y)|y.0>9&&y.1)  {
                map[x][y].1=false;
                map.get_mut((x as i32-1) as usize).and_then(|b|b.get_mut(y)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut((x as i32-1) as usize).and_then(|b|b.get_mut((y as i32-1) as usize)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut((x as i32-1) as usize).and_then(|b|b.get_mut(y+1)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut(x+1).and_then(|b|b.get_mut(y)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut(x+1).and_then(|b|b.get_mut((y as i32-1) as usize)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut(x+1).and_then(|b|b.get_mut(y+1)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut(x).and_then(|b|b.get_mut((y as i32-1) as usize)).and_then(|c|{c.0+=1;Some(())});
                map.get_mut(x).and_then(|b|b.get_mut(y+1)).and_then(|c|{c.0+=1;Some(())});
            }
        }
    }
    map.iter_mut().for_each(|a|a.iter_mut().filter(|(_,b)|!b).for_each(|(c,_)|*c=0));
    map.iter().map(|a|a.iter().filter(|(_,b)|!b).count() as u32).sum::<u32>()
}

pub fn solve(){
    let input = fs::read_to_string("data/day11.dat").expect("file day11 fehlt");
    let mut map:Vec<Vec<(u32,bool)>> = input.lines().map(|x|x.chars().map(|y|(y.to_digit(10).unwrap(),true)).collect()).collect();
    let mut count=0;
    let mut i =0;
    while !map.iter().all(|a|a.iter().all(|(_,b)|!b)) {
        count+=step(&mut map);
        i+=1;
        if i==100 {
            println!("after 100 steps it flasched {} times",count);
        }
    }
    println!("syncroiset after {} steps",i )


}