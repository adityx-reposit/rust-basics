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

struct Rect<T>{
    height:T,
    width:T
}

impl<T:std::ops::Mul<Output = T>+Copy> Rect<T>{
    fn findArea(&self)->T{
        return self.height*self.width; 
    }
}

fn main(){
     let r1=Rect{
        height:20,
        width:20
     };
     let r2=Rect{
        height:40.09098,
        width:87.456778
     };
    
     let data=r1.findArea() ;
     println!("{}",data);   
}




