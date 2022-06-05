/// #######################################################
///  EVE Online Wormhole Tracker
/// 



struct Wormhole {

}

struct Anomaly {

}

struct Signature {
    id: [u8;6],
    sigtype: SigType
}

enum SigType {
    Wormhole(String),
    Combat(String),
    Gas(String),
    Ore(String),
    Relic(String),
    Data(String),
}

fn main() {
    println!("Hello, world!");
}