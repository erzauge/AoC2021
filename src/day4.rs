use std::fs;
use std::str::FromStr;

struct Bingo{
    board :Vec<Vec<(u32,bool)>>
}

impl FromStr for Bingo {
    type Err =std::num::ParseIntError;
    fn from_str(bingo_board: &str) ->Result<Self,Self::Err>{
        let board = bingo_board.lines().map(|x| x.split_whitespace().map(|x|match x.parse::<u32>(){Ok(x)=>Ok((x,false)),Err(e)=>Err(e)}).collect()).collect::<Result<Vec<Vec<(u32,bool)>>,_>>()?;
        Ok(Bingo{board})
    }
}

impl Bingo {
    fn set(&mut self,num:u32) {
        self.board.iter_mut().for_each(|x| x.iter_mut().for_each(|(  c,m) | *m|=*c==num))
    }
    
    fn bingo(&self) -> bool{
        let h=self.board.iter().any(|x| x.iter().all(|&(_,x)|x)); //hroizontal
        let mut v = false;
        for i in 0..self.board.len(){
            v|=self.board.iter().all(|x |x[i].1)
        }
        h||v
    }

    fn score(&self) -> u32 {
        self.board.iter().fold(0, |sum,x| sum+x.iter().fold(0, |sum2,y|if !y.1 {sum2+y.0} else {sum2}))
    }
}

pub fn solve() {
    let input = fs::read_to_string("data/day4.dat").expect("fiel day4 fehlt");
    let num:Vec<u32> = input.lines().next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    let mut bords:Vec<Bingo> = input.split("\n\n").skip(1).map(|x| x.parse::<Bingo>()).collect::<Result<Vec<Bingo>,_>>().unwrap();
    for &i in num.iter() {
        bords.iter_mut().for_each(|x| (*x).set(i));
        // println!("{}",i);
        if bords.iter().any(|x|x.bingo()) {
            println!("score {}",bords.iter().find(|x|x.bingo()).unwrap().score()*i);
            bords=bords.into_iter().filter(|x|!x.bingo()).collect()
        }
    }
}