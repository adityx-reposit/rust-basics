// struct User{
//     username:String
// }

// fn main() {
//     let resposnse=compare(210, 60);
//     println!("{}",resposnse)
// }



// fn printit<X:std::fmt::Display>(a:X,b:X){
//     println!("{}",a);
//     println!("{}",b);
// }


// fn sum<X:std::ops::Add <Output = X>>(a:X,b:X)->X{
//      return  a+b;
// }

// fn compare<X:std::cmp::PartialOrd>(a:X,b:X)->X{
//     if a>b{
//         return a;
//     }
//     return b;
// }

//generating traits over structs 

// #[derive(Copy,Clone)]

// struct Rect<T>{
//     height:T,
//     width:T


// }

// impl<T:std::ops::Mul<Output = T>+Copy > Rect<T>{
//     fn area(&self)->T{
      
//         return self.height *self.width;  
// }
// }

// fn main(){                    
//  let r1=Rect{
//     height:30,
//     width:20
//  };

//  let r2=Rect{
//     height:30.00004,
//     width:450.0030
//  };

// let r3=Rect{
//    height:2000,
//    width:30000
// };


// let dataaaaa=r3.area();
// println!("{}",dataaaaa);

//  let data=r1.area();
//  println!("{}",data);

//  let dataa=r2.area();
//  print!("{}",dataa);
// }




// generics over enum
// enum Option<T> {
//    Some(T),
//    None
// }

// fn main(){
//    let x:std::option::Option<f64>=Some(-40.322);


//     match x = {
//         Some(30.0988) =>print!("done"),
//         None => print!("error ocuured")
//     }


// }



// learning traits for trait boundaries

use std::string;

struct Rect{
    height:f32,
    width:f32
}

impl Shapes for Rect{
    fn area(&self)->f32{
       return self.height* self.width; 
    }
}


struct Circle{
    radius:f32
}
impl Shapes for Circle{
    fn area(&self)->f32{
       return  self.radius *self.radius; 
    }
}

trait Shapes{
    fn area(&self)->f32;
}


fn area <T:Shapes>(r:T){
   println!("{}",r.area())
}

fn main(){
  
let r=Rect{
    height:40.09,
    width:90.45

};
let c=Circle{
    radius:23.76
};
area(r);
area(c);


area(c);
area(r);
}