use std::fs;
use std::str::FromStr;


struct SegmentdDisplay {
    digits : Vec<String>,
    code : Vec<String>,
    maping : Vec<String>,
}

impl FromStr for SegmentdDisplay {
    type Err =std::num::ParseIntError;
    fn from_str(segmentd_display: &str) ->Result<Self,Self::Err>{
        let mut iter = segmentd_display.split("|");
        let digits=iter.next().expect("error split").split_whitespace().map(|x|x.to_string()).collect();
        let code=iter.next().expect("error split").split_whitespace().map(|x|x.to_string()).collect();
        Ok(SegmentdDisplay{digits,code,maping:vec!["".to_string();10]})
    }
}

impl SegmentdDisplay {
    fn solve_mapping(&mut self){
        self.maping[1]=self.digits.iter().find(|x|x.len()==2).unwrap().clone();
        self.maping[4]=self.digits.iter().find(|x|x.len()==4).unwrap().clone();
        self.maping[7]=self.digits.iter().find(|x|x.len()==3).unwrap().clone();
        self.maping[8]=self.digits.iter().find(|x|x.len()==7).unwrap().clone();
        self.maping[3]=self.digits.iter().filter(|x|x.len()==5).find(|x|self.maping[1].chars().all(|y|x.contains(y))).unwrap().clone();
        self.maping[9]=self.digits.iter().filter(|x|x.len()==6).find(|x|self.maping[3].chars().all(|y|x.contains(y))).unwrap().clone();
        self.maping[0]=self.digits.iter().filter(|x|x.len()==6).find(|x|self.maping[1].chars().all(|y|x.contains(y))&&**x!=self.maping[9]).unwrap().clone();
        self.maping[6]=self.digits.iter().filter(|x|x.len()==6).find(|x|(**x!=self.maping[0])&&(**x!=self.maping[9])).unwrap().clone();
        self.maping[5]=self.digits.iter().filter(|x|x.len()==5).find(|x|x.chars().all(|y|self.maping[6].contains(y))).unwrap().clone();
        self.maping[2]=self.digits.iter().filter(|x|x.len()==5).find(|x|**x!=self.maping[5]&&**x!=self.maping[3]).unwrap().clone();
    }
    fn decode(&self) -> u32 {
        self.code.iter().fold("".to_string(), |cat,x| cat+&self.maping.iter().position(|y|*y==*x).unwrap().to_string()).parse().unwrap()
    }

    fn sort(&mut self){
        self.digits.iter_mut().for_each(|x|{
            let mut s=x.chars().collect::<Vec<char>>();
            s.sort();
            *x=s.into_iter().collect()
        });
        self.code.iter_mut().for_each(|x|{
            let mut s=x.chars().collect::<Vec<char>>();
            s.sort();
            *x=s.into_iter().collect()
        });
        self.maping.iter_mut().for_each(|x|{
            let mut s=x.chars().collect::<Vec<char>>();
            s.sort();
            *x=s.into_iter().collect()
        });
    }
}

pub fn solve(){
    let input = fs::read_to_string("data/day8.dat").expect("file day8 fehlt");
    let mut displais:Vec<SegmentdDisplay> = input.lines().map(|x|x.parse::<SegmentdDisplay>().expect("feherler in segment")).collect();

    let num_of_1478 =displais.iter().fold(0, |sum,x|sum+x.code.iter().filter(|&x|(x.len()>=2)&&(x.len()<=4)||(x.len()==7)).count());

    println!("# {}",num_of_1478);
    displais.iter_mut().for_each(|x|x.sort());
    displais.iter_mut().for_each(|x|x.solve_mapping());
    let sum = displais.iter().map(|x|x.decode()).sum::<u32>();
    println!("# {}",sum);
}