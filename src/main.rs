use serde ::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug,Clone)]
struct User{
    username:String,
    password:String,
    age:u32

}





    

fn main() {

let user1=User{
    username:String::from("Aditya"),
    password:String::from("12345"),
    age:21
};


let serialize_string=  serde_json::to_string(&user1);


match serialize_string{
    Ok(str)=>println!("{}",str),
    Err(e)=>println!("{}",e)
}

let s=  String::from("{\"username\":\"aditya\",\"password\":\"123456\"}");
let u:Result<User,serde_json::Error> =serde_json::from_str(&s);
 
 let data = u.unwrap();
 println!("{:?}",data);
    
}
