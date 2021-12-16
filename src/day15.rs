use std::fs;
use std::collections::{BinaryHeap,HashSet};
use std::cmp::Ordering;
#[derive(Copy, Clone, Eq, PartialEq)]
struct State{
    risk: u32,
    pos: (usize,usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.risk.cmp(&self.risk)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn min<T:Ord>(a:Option<T>,b:Option<T>)->Option<T>{
    if a.is_none(){
        return b;
    }
    else if b.is_none() {
        return a;
    }
    else {
        return std::cmp::min(a, b);
    }
}

fn adjesent(a:(usize,usize),l:usize)->Vec<(usize,usize)>{
    let mut r= vec![];
    if a.0>0 {r.push((a.0-1,a.1));}
    if a.1>0 {r.push((a.0,a.1-1));}
    if a.0<(l-1) {r.push((a.0+1,a.1));}
    if a.1<(l-1) {r.push((a.0,a.1+1));}
    r
}

fn lowers_risk_path(map:&Vec<Vec<u32>>)->u32{
    let l=map.len();
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX;l];l];
    let mut p =(0,0);
    let mut heap=BinaryHeap::new();
    heap.push(State{risk:0,pos:p});
    dist[0][0]=0;
    while let Some(State { risk, pos })=heap.pop() {
        if pos==(l-1,l-1) {break;}
        if  risk>dist[pos.0][pos.1] {continue;}
        for a in adjesent(pos,l) {
            let new = State{risk:risk+map[a.0][a.1],pos:a};
            if new.risk<dist[a.0][a.1] {
                heap.push(new);
                dist[a.0][a.1]=new.risk
            }
        }
    }
    dist[l-1][l-1]
}

pub fn solve() {
    let input = fs::read_to_string("data/day15.dat").expect("file day15 fehlt");
    let map:Vec<Vec<u32>> = input.lines().map(|a|a.chars().map(|b|b.to_digit(10).unwrap()).collect()).collect();
    
    println!("smale {}",lowers_risk_path(&map));
    let map_big_x:Vec<Vec<u32>>=map.iter().map(|x|(0..5).fold(vec!(), |mut vec:Vec<u32>,a|{vec.append(&mut x.iter().map(|b|(b+a-1)%9+1).collect());vec})).collect();
    let map_big: Vec<Vec<u32>> = (0..5).fold(vec![], |mut vec,a|{vec.append(&mut map_big_x.iter().map(|b|b.iter().map(|b|(b+a-1)%9+1).collect()).collect());vec});
    println!("big {}",lowers_risk_path(&map_big));
}

