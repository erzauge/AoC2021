use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;
use std::collections::HashMap;

struct Inserts{
    pattern:String,
    insert:Vec<String>,
}

impl FromStr for Inserts {
    type Err =std::num::ParseIntError;
    fn from_str(insert: &str) ->Result<Self,Self::Err>{
        let mut iter=insert.split(" -> ");
        let pattern =iter.next().unwrap().to_string();
        let i= iter.next().unwrap().to_string();
        let insert =vec![pattern.chars().next().unwrap().to_string()+&i,i+&pattern.chars().nth(1).unwrap().to_string()];
        Ok(Inserts{pattern,insert})
    }
}

pub fn solve() {
    let input = fs::read_to_string("data/day14.dat").expect("file day14 fehlt");
    let mut iter =input.split("\n\n");
    let polymer = iter.next().unwrap().to_string();
    let patterns :Vec<Inserts> = iter.next().unwrap().lines().map(|x|x.parse::<Inserts>().unwrap()).collect();
    let mut paire_map = polymer.chars().collect::<Vec<char>>().windows(2).fold(HashMap::new(), |mut map,c|{*map.entry(String::from_iter(c)).or_insert(0i128)+=1; map});

    for _ in 0..10 {
        let mut new_paire_map =paire_map.clone();
        for i in patterns.iter(){
            if let Some(x)  =paire_map.get_mut(&i.pattern)  {
                i.insert.iter().for_each(|a| *new_paire_map.entry(a.clone()).or_insert(0)+=*x);
                *new_paire_map.entry(i.pattern.clone()).or_insert(0)-=*x;
            }
        }
        paire_map=new_paire_map;
    }
    {
        let mut counts =paire_map.iter().fold(HashMap::new(),|mut hist,(p,c)|{p.chars().for_each(|i|*hist.entry(i).or_insert(0)+=*c as u128);hist});
        *counts.get_mut(&polymer.chars().next().unwrap()).unwrap()+=1;
        *counts.get_mut(&polymer.chars().last().unwrap()).unwrap()+=1;
        let mut quantities:Vec<u128> = counts.values().map(|i|*i).collect();
        quantities.sort();
        println!("after 10 steps {}",quantities.last().unwrap()/2-quantities.first().unwrap()/2);
    }

    for _ in 0..30 {
        let mut new_paire_map =paire_map.clone();
        for i in patterns.iter(){
            if let Some(x)  =paire_map.get_mut(&i.pattern)  {
                i.insert.iter().for_each(|a| *new_paire_map.entry(a.clone()).or_insert(0)+=*x);
                *new_paire_map.entry(i.pattern.clone()).or_insert(0)-=*x;
            }
        }
        paire_map=new_paire_map;
    }
    {
        let mut counts =paire_map.iter().fold(HashMap::new(),|mut hist,(p,c)|{p.chars().for_each(|i|*hist.entry(i).or_insert(0)+=*c as u128);hist});
        *counts.get_mut(&polymer.chars().next().unwrap()).unwrap()+=1;
        *counts.get_mut(&polymer.chars().last().unwrap()).unwrap()+=1;
        let mut quantities:Vec<u128> = counts.values().map(|i|*i).collect();
        quantities.sort();
        println!("after 40 steps {}",quantities.last().unwrap()/2-quantities.first().unwrap()/2);
    }
}