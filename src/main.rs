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

use core::fmt;
use std::string;

use borsh::{error, BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize,BorshSerialize,Clone,Debug)]

struct User{
    username:String,
    password:String,
    age:u32

}




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

fn main(){
   longest_str(String::from("Adityaaaaaa"), String::from("Yadacavsgsgsg"));
}

fn longest_str(s1:String , s2:String){
    if s1.len()> s2.len() {
        println!("longest string is {}",s1);
    }
    else{
        println!("longest string is {}",s2);
    }
}