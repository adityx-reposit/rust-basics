

fn main() {

   let gener=general(20,90909090);
    println!("{}",gener);
   
}



fn general<A:std::ops::Add<Output=A>>(a:A,b:A)->A{
    return a+b
}
