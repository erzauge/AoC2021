use std::fs;

pub fn solve(){
    let input = fs::read_to_string("data/day6.dat").expect("file day6 fehlt");
    let lanternfish: Vec<u32> = input.lines().next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    let mut states: [u64;9] = [0;9];
    for l in lanternfish {
        states[l as usize]+=1;
    }
    for i in 0..256 {
        if i==80 {
            println!("# lanternfish after 80 days {}",states.iter().sum::<u64>());
        } 
        states.rotate_left(1);
        states[6]+=states[8];
    }

    println!("# lanternfish after 256 days {}",states.iter().sum::<u64>());
}