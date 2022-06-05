/// #######################################################
///  EVE Online Wormhole Tracker
/// 

// struct Wormhole {
//     id: [u8;6],
//     whtype: 
// }

// struct Signature {
//     id: [u8;6],
//     name: String,
//     sigtype: SigType,
// }
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::collections::{HashMap};
type WormHoleType = HashMap<String, Wormhole>;
enum Signature {
    Combat      {id: String, name: String, lifetime: usize},
    Gas         {id: String, name: String, lifetime: usize},
    Ore         {id: String, name: String, lifetime: usize},
    Relic       {id: String, name: String, lifetime: usize},
    Data        {id: String, name: String, lifetime: usize},
    Wormhole    {id: String, name: String, lifetime: usize, age: usize, wormhole_type: String, destination: String},
}

impl Signature {
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Wormhole {
    destination: String,
    ttl: usize,
}

fn load_whdata() -> Result<WormHoleType, Box<dyn Error>> {
    let file = File::open("wormholes.json").expect("oops");
    let reader = BufReader::new(file);

    let jdata: WormHoleType = serde_json::from_reader(reader)?;
    Ok(jdata)
}

fn main() {
    let s = Signature::Wormhole {
        id: "GOM-178".to_string(),
        name: "J233630".to_string(),
        lifetime: 128,
        age: 0,
        wormhole_type: "H900".to_string(),
        destination: "ASDF".to_string()};
    
    let w = load_whdata();
    println!("{:?}", w);
}