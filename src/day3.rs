use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("data/day3.dat").unwrap();
    let reader = BufReader::new(file);
    let mut c=vec![0;12];
    let mut count=0;
    let mut r="".to_string();
    let mut list :Vec<String>=vec!(); 
    for line in reader.lines(){
        let line = line.unwrap();
        count+=1;
        for i in 0..12 {
            if line.as_bytes()[i]==('1' as u8) {
                c[i]+=1;
            }
        }
        list.push(line);
    }
    for i in 0..12 {
        if c[i]>=(count/2) {
            r+="1";
        }
        else {
            r+="0";
        }
    }
    let x=u32::from_str_radix(&r, 2).unwrap();
    println!("gamma {}",x);
    println!("epsilon {}",!(x)&4095);
    println!("power {}",(!(x)&4095)*x);
    let mut o2_iter=list.clone().into_iter();
    for i in 0..12 {
        let sum = o2_iter.clone().fold(0, |sum,j| {
            if j.as_bytes()[i]==('1' as u8) {
                sum+1
            }
            else {
                sum
            }
        });
        let len=o2_iter.len();
        let char=if (sum*2)>=len {'1'} else {'0'} as  u8;
        o2_iter=o2_iter.filter(|x| x.as_bytes()[i]==char).collect::<Vec<String>>().into_iter();
    }
    let o2= u32::from_str_radix(&o2_iter.next().unwrap(), 2).unwrap();
    println!(" o2 {}",o2);

    let mut co2_iter=list.clone().into_iter();
    for i in 0..12 {
        let sum = co2_iter.clone().fold(0, |sum,j| {
            if j.as_bytes()[i]==('1' as u8) {
                sum+1
            }
            else {
                sum
            }
        });
        let len=co2_iter.len();
        let char=if (sum*2)>=len {'0'} else {'1'} as  u8;
        co2_iter=co2_iter.filter(|x| x.as_bytes()[i]==char).collect::<Vec<String>>().into_iter();
        if co2_iter.len()==1 {break;}
    }
    let co2= u32::from_str_radix(&co2_iter.next().unwrap(), 2).unwrap();
    println!("co2 {}",co2);
    println!("life support {}",co2*o2);
}