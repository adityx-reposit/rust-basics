// use std::string;

// use serde ::{Serialize,Deserialize};


// #[derive(Serialize,Deserialize,Debug,Clone)]
// struct User{
//     username:String,
//     password:String,
//     age:u32

// }





    

// fn main() {

// let user1=User{
//     username:String::from("Aditya"),
//     password:String::from("12345"),
//     age:21
// };


// let serialize_string=  serde_json::to_string(&user1);


// match serialize_string{
//     Ok(str)=>println!("{}",str),
//     Err(e)=>println!("{}",e)
// }

// // Deserialize JSON back to User struct
// let deserialize_result: Result<User, serde_json::Error> = serde_json::from_str(r#"{"username":"Aditya","password":"12345","age":21}"#);

// match deserialize_result {
//     Ok(user) => println!("Deserialized user: {}", user.age),
//     Err(e) => println!("Deserialization error: {}", e)
// }
// }

//borsh started

// use core::fmt;
// use std::string;

// use borsh::{error, BorshDeserialize, BorshSerialize};


// #[derive(BorshDeserialize,BorshSerialize,Clone,Debug)]

// struct User{
//     username:String,
//     password:String,
//     age:u32

// }




// fn main(){
//   let u = User{
//     username:String::from("Aditya"),
//     password:String::from("123123"),
//     age:21
//   };

// let mut  v=Vec::new();
// let ans = u.serialize(&mut v);

// println!("{:?}",v);

// let data =User::try_from_slice(&v).unwrap();
// println!("{}",data.age)
// }


//LIFETIME
// struct User{
//     username:String,
//     password:String
// }
// fn main(){
//     let st1=String::from("Adityaa");
//     let st2=String::from("yadav");
//     let ans ;
//     {

//         let st3=String::from("yadav");
//          ans= longest_st(&st1,&st2,&st3);
//          println!("{}",ans);
//     }
// }

// fn longest_st<'a, 'b>(s1:& 'a String , s2:& 'a String, s3: & 'b String)-> &'a String{
//     if s1.len()> s2.len() {
//         return s1;
//     }
//     else{
//         return s2;
//     }
// }



fn main(){
    let ans="";
    let s1= String::from("Aditya");
    {
        let s2= String::from("Yadav");
        let ans= gather(&s1, &s2);
    }
    println!("{}",ans);
}


fn gather<'a, 'b >(St1:&'a String, St2:& 'b  String)->& 'a String{
     if St1.len() > St2.len(){
         return St1;
     }
     else{
        return St2;
     }
}