use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Point{
    x: i32,
    y:i32,
}
struct Line{
    start:Point,
    end:Point,
}

impl FromStr for Point {
    type Err =std::num::ParseIntError;
    fn from_str(point: &str) ->Result<Self,Self::Err>{
        let mut point =point.split(",");
        let x=point.next().expect("no x").parse()?;
        let y=point.next().expect("no y").parse()?;
        Ok(Point{x,y})
    }
}

impl FromStr for Line {
    type Err =std::num::ParseIntError;
    fn from_str(line: &str) ->Result<Self,Self::Err>{
        let mut line =line.split(" -> ");
        let start= line.next().expect("no Starting point").parse()?;

        let end= line.next().expect("no ending ponint").parse()?;

        Ok(Line{start,end})
    }
}

fn gcd(a:u32,b:u32)->u32{
    let mut a=a;
    let mut b=b;
    if a==0{
        return b;
    }
    while b!=0 {
        let h =a%b;
        a=b;
        b=h;
    }
    return a;
}

impl Line {
    fn h_or_v(&self) -> bool {
        (self.start.x==self.end.x)||(self.start.y==self.end.y)
    }
    fn vents(&self) -> Vec<Point> {
        let mut dx=self.end.x-self.start.x;
        let mut dy=self.end.y-self.start.y;
        let gcd =gcd(dx.abs() as u32, dy.abs() as u32) as i32;
        dx/=gcd;
        dy/=gcd;
        (0..=gcd).map(|i| Point{x:self.start.x+dx*i,y:self.start.y+dy*i}).collect()
    }
}

pub fn solve(){
    let input = fs::read_to_string("data/day5.dat").expect("file day5 fehlt");
    let lines: Vec<Line> = input.lines().map(|x|x.parse::<Line>().unwrap()).collect();
    let mut dic_vents = HashMap::<Point,u32>::new();
    for l in lines.iter().filter(|x|x.h_or_v()){
        for v in l.vents() {
            match dic_vents.get_mut(&v) {
                Some(x) =>*x+=1,
                None => {dic_vents.insert(v, 1);}
            }
        }
    }
    
    println!("anzal an vents in mehr als einer  horzontalen oder vetikalen line: {}",dic_vents.iter().filter(|(_,x)|**x>1).count());

    for l in lines.iter().filter(|x|!x.h_or_v()){
        for v in l.vents() {
            match dic_vents.get_mut(&v) {
                Some(x) =>*x+=1,
                None => {dic_vents.insert(v, 1);}
            }
        }
    }
    println!("anzal an vents in mehr als einer line: {}",dic_vents.iter().filter(|(_,x)|**x>1).count());
}