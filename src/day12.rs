use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

struct Graph{
    edge_list:HashMap<String,Vec<String>>
}

impl Graph {
    fn add_edge(&mut self,a:String,b:String){
        self.edge_list.entry(a.clone()).or_insert(vec!()).push(b.clone());
        self.edge_list.entry(b.clone()).or_insert(vec!()).push(a.clone());
    }
    fn count_simpel_path_smale(&self,a:String,b:String,visited:&mut Vec<String>)->u32{
        if a==b {
            return 1;
        }
        let mut sum=0;
        if a.to_lowercase()==a {
            visited.push(a.clone())
        }
        for e in self.edge_list.get(&a).unwrap().iter().filter(|x|!visited.contains(x)) {
            sum+=self.count_simpel_path_smale(e.clone(), b.clone(),&mut visited.clone());
        }
        if a.to_lowercase()==a {
            visited.pop();
        }
        return sum;
        
    }

    fn count_simpel_path_smale_one_twice(&self,a:String,b:String,visited:&mut Vec<String>,d:String)->u32{
        if a==b {
            if visited.contains(&d)||d.is_empty(){
                return 1;
            }
            return 0;
        }
        let mut sum=0;
        if a.to_lowercase()==a {
            if a!="start"&&d.is_empty(){
                for e in self.edge_list.get(&a).unwrap().iter().filter(|x|!visited.contains(x)) {
                    sum+=self.count_simpel_path_smale_one_twice(e.clone(), b.clone(),&mut visited.clone(),a.clone());
                }
            }
            visited.push(a.clone())
        }
        for e in self.edge_list.get(&a).unwrap().iter().filter(|x|!visited.contains(x)) {
            sum+=self.count_simpel_path_smale_one_twice(e.clone(), b.clone(),&mut visited.clone(),d.clone());
        }
        if a.to_lowercase()==a {
            visited.pop();
        }
        return sum;
        
    }
}

impl FromStr for Graph {
    type Err =std::num::ParseIntError;
    fn from_str(graph: &str) ->Result<Self,Self::Err>{
        let mut g = Graph{edge_list:HashMap::new()};
        for l in graph.lines()  {
            let mut l = l.split('-');
            let a=l.next().unwrap();
            let b =l.next().unwrap();
            g.add_edge(a.to_string(), b.to_string());
        }
        Ok(g)
    }
}

pub fn solve(){
    let input = fs::read_to_string("data/day12.dat").expect("file day12 fehlt");
    let g = input.parse::<Graph>().unwrap();
    println!("# simpel path of smale caves start to end {}",g.count_simpel_path_smale("start".to_string(), "end".to_string(),&mut vec!()));
    println!("# simpel path of smale with on twice caves start to end {}",g.count_simpel_path_smale_one_twice("start".to_string(), "end".to_string(),&mut vec!(),"".to_string()));
}