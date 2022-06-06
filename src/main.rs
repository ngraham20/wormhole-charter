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

use wormhole_charter::{Signature, sig};
// trait Signature {
//     fn id(&self) -> String;
//     fn name(&self) -> String;
//     fn lifetime(&self) -> usize;
// }

struct CombatSignature {
    id: String,
    name: String,
    lifetime: usize,
}
sig!(CombatSignature);
// impl Signature for CombatSignature {
//     fn id(&self) -> String {
//         self.id.clone()
//     }
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn lifetime(&self) -> usize {
//         self.lifetime
//     }
// }

// custom macro, expands to implement default of Signature trait

#[derive(Serialize, Deserialize, Debug)]
struct Wormhole {
    wormhole_type: String,
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
    
    let w = load_whdata();
    println!("{:?}", w);
}