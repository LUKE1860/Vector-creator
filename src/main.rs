use std::io;
fn first(){
println!("Enter letters");
let mut r=Vec::<String>::new();
let mut c=0;
while c!=999{
let mut b=String::new();
io::stdin().read_line(&mut b).expect("R");
let x:String=b.trim().parse().expect("R");
r.push(b);
c+=1;
match x.trim().as_ref(){
"end"=>break,
_=>continue,
}
}
for vector in r.iter(){
    println!("{}",vector);
}
}
fn second(){
    println!("Enter letters");
    let mut r=Vec::<f64>::new();
    let mut c=0;
    while c!=999{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:f64=b.trim().parse().expect("C");
    r.push(x);
    c+=1;
    match x{
    0.125=>break,
    _=>continue,
    }
}
    for vector in r.iter(){
        println!("{}",vector);
    }
}
fn third(){
    println!("Enter letters");
    let mut r=Vec::<i32>::new();
    let mut c=0;
    while c!=999{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:i32=b.trim().parse().expect("S");
    r.push(x);
    c+=1;
    if x==0{
    break;
    }
    else{
    continue;
    }
    }
    for vector in r.iter(){
        println!("{}",vector);
    }
}
fn fourth(){
    println!("Enter letters");
    let mut r=Vec::<u32>::new();
    let mut c=0;
    while c!=999{
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("R");
    let x:u32=b.trim().parse().expect("S");
    r.push(x);
    c+=1;
    if x==0{
        break;
        }
        else{
        continue;
        }
    }
    for vector in r.iter(){
        println!("{}",vector);
    }
}
fn main(){
let mut a=String::new();
println!("Create vectors by type");
io::stdin().read_line(&mut a).expect("R");
match a.trim().as_ref(){
"S"=>first(),
"D"=>second(),
"I"=>third(),
"U"=>fourth(),
_=>panic!("Unreachable"),
}
}