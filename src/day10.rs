use std::fs;


fn closing_brakets(x:char)->Option<char>{
    match x {
        '('=>Some(')'),
        '['=>Some(']'),
        '{'=>Some('}'),
        '<'=>Some('>'),
        _=> None,
    }
}

fn scoreing(x:char)->u32{
    match x {
        ')'=>3,
        ']'=>57,
        '}'=>1197,
        '>'=>25137,
        _=> 0,
    }
}
fn scoreing2(x:char)->u32{
    match x {
        ')'=>1,
        ']'=>2,
        '}'=>3,
        '>'=>4,
        _=> 0,
    }
}

fn coruptline( l:&str )->u32{
    let mut stack=vec![];
    let opening =vec!['(','[','{','<'];
    for c in l.chars(){
        if opening.contains(&c){
            stack.push(c);
        }
        else if Some(c)==stack.last().and_then(|x|closing_brakets(*x)) {
            stack.pop();
        }
        else {
            return scoreing(c);
        }
    }
    return 0;
}

fn missing_closing( l:&str )->u64{
    let mut stack=vec![];
    let opening =vec!['(','[','{','<'];
    for c in l.chars(){
        if opening.contains(&c){
            stack.push(c);
        }
        else if Some(c)==stack.last().and_then(|x|closing_brakets(*x)) {
            stack.pop();
        }
        else {
            panic!("error corupt line")
        }
    }
    let mut score:u64=0;
    while let Some(x) = stack.pop(){
        score*=5;
        score+=scoreing2(closing_brakets(x).unwrap()) as u64;
    }
    return score;
}

pub fn solve(){
    let input = fs::read_to_string("data/day10.dat").expect("file day10 fehlt");
    let lines :Vec<&str> = input.lines().collect();
    let score= lines.iter().map(|x| coruptline(x)).sum::<u32>();
    println!("first score {}",score);
    let mut compliet_lines:Vec<u64>=lines.iter().filter(|x|coruptline(x)==0).map(|x|missing_closing(x)).collect();
    compliet_lines.sort();
    println!("mid Autocomplete score {}",compliet_lines[compliet_lines.len()/2]);

}