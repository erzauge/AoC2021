use std::fs;

pub fn solve() {
    let input = fs::read_to_string("data/day7.dat").expect("file day7 fehlt");
    let mut crab: Vec<u32> = input.lines().next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    crab.sort();
    let mut mid;
    if crab.len()%2==1
    {
        mid = crab[crab.len()/2] as i32;
    }
    else {
        mid = (crab[crab.len() / 2 - 1] as i32 + crab[crab.len() / 2] as i32) /2
    }
    let fuel: u32 = crab.iter().map(|x|(*x as i32 -mid as i32).abs() as u32).sum::<u32>();
    println!("first fuel {} at {}",fuel,mid);
    let mut count=0;
    let mut grad_old=0;
    let mut fuel2=0;
    loop {
        
        let grad = crab.iter().map(|x|if (*x as i32 -mid as i32)!=0 {*x as i32 -mid as i32+(*x as i32 -mid as i32)/(*x as i32 -mid as i32).abs()}else{0}).sum::<i32>();
        if grad.signum()*grad_old==-1||count>200 {break;}
        fuel2 = crab.iter().map(|x|((*x as i64 -mid as i64).pow(2)+(*x as i64 -mid as i64).abs())/2).sum::<i64>();
        mid+=grad.signum();
        count+=1;
        grad_old=grad.signum()
    }

    println!("secend fuel {} at {}",fuel2,mid);
}