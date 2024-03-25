use std::io::stdin;

fn gcd(mut a: u32, mut b:u32)->u32{
    if a*b == 0 {
        return 0
    }
    let mut c ;
    while a%b != 0 {
        c = a%b;
        a = b;
        b = c;
    }
    return b;
}

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let mut line = input.split_whitespace();
    let mut list :Vec<u32> = Vec::new();
    loop{
        match line.next(){
            Some(num)=>{list.push(num.parse().unwrap())},
            None=>{break}
        };
    }
    println!("{:?}",list);
    println!("gcd(a,b) = {}", gcd(list[0], list[1]));

}