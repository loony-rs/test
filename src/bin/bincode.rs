use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct User {
    fname: String,
    lname: String,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Users(Vec<User>);

fn main() {
    let config = config::standard();

    let users = Users(vec![User { fname: String::from("Sankar"), lname: String::from("Boro") }, User { fname: String::from("Arun"), lname: String::from("Das") }]);

    let encoded: Vec<u8> = bincode::encode_to_vec(&users, config).unwrap();

    println!("encoded: {:?}", encoded);

    let (decoded, len): (Users, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

    println!("decoded: {:?}", decoded);
}