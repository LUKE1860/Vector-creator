use std::io;
fn first(){
println!("Enter letters");
let mut r=Vec::<String>::new();
let mut d=String::new();
loop{
let mut b=String::new();
io::stdin().read_line(&mut b).expect("R");
let x:String=b.trim().parse().expect("R");
r.push(b);
match x.trim(){
"end"=>break,
_=>continue,
}
}
println!("Do you want to sort a vector?");
    io::stdin().read_line(&mut d).expect("W");
    if d.contains("Yes"){
    r.sort();
    for vector in r.iter(){
        println!("{}",vector);
    }
    }
    else if d.contains("No"){
        for vector in r.iter(){
            println!("{}",vector);
        }
    }
}
fn second(){
    println!("Enter numbers");
    let mut r=Vec::<f64>::new();
    let mut d=String::new();
    loop{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:f64=b.trim().parse().expect("C");
    r.push(x);
    match x{
    0.0=>break,
    _=>continue,
    }
}
println!("Do you want to sort a vector?");
io::stdin().read_line(&mut d).expect("W");
if d.contains("Yes"){
    r.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for vector in r.iter(){
    println!("{}",vector);
    }
}
else if d.contains("No"){
    for vector in r.iter(){
        println!("{}",vector);
    }
}
}
fn third(){
    println!("Enter numbers");
    let mut r=Vec::<i32>::new();
    let mut d=String::new();
    loop{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:i32=b.trim().parse().expect("S");
    r.push(x);
    if x==0{
    break;
    }
    else{
    continue;
    }
    }
    println!("Do you want to sort a vector?");
    io::stdin().read_line(&mut d).expect("W");
    if d.contains("Yes"){
    r.sort();
    for vector in r.iter(){
        println!("{}",vector);
    }
    }
    else if d.contains("No"){
        for vector in r.iter(){
            println!("{}",vector);
        }
    }
}
fn fourth(){
    println!("Enter numbers");
    let mut r=Vec::<u32>::new();
    let mut d=String::new();
    loop{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:u32=b.trim().parse().expect("S");
    r.push(x);
    if x==0{
        break;
        }
        else{
        continue;
        }
    }
    println!("Do you want to sort a vector?");
    io::stdin().read_line(&mut d).expect("W");
    if d.contains("Yes"){
    r.sort();
    for vector in r.iter(){
        println!("{}",vector);
    }
}
    else if d.contains("No"){
    for vector in r.iter(){
    println!("{}",vector);
        }
    }
    }

fn main(){
let mut a=String::new();
println!("Create vectors by type");
io::stdin().read_line(&mut a).expect("R");
match a.trim(){
"S"=>first(),
"D"=>second(),
"I"=>third(),
"U"=>fourth(),
_=>panic!("Unreachable"),
}
}