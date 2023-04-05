use std::{cmp::{min, max}, collections::BTreeSet};

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct NevskyLocs {
    name: String,
    #[serde(rename = "type")]
    loc_type: String,
    region: String,
    adjacent: Vec<usize>,
    adjacent_by_trackway: Vec<usize>,
    adjacent_by_waterway: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum WayType {
    Track,
    Water
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Way {
    side_a: String,
    side_b: String,
    way: WayType,
}

// {"name":"Reval","type":"bishopric","stronghold":3,"walls":4,"vp":2,"region":"Danish Estonia","ways":[[5,25],[3,32]],"box":{"x":150,"y":891,"w":52,"h":23},"adjacent":[3,5],"adjacent_by_trackway":[3,5],"adjacent_by_waterway":[],"trackways":[[3,32],[5,25]],"waterways":[]},

fn main() {

    let mut locs = Vec::new();
    let mut ways = BTreeSet::new();


    let mut locs_json = {
        // Load the first file into a string.
        let text = std::fs::read_to_string("/workspaces/wargames/nevsky/src/levy_and_campaign/nevsky_loc.json").unwrap();

        serde_json::from_str::<Vec<NevskyLocs>>(&text).unwrap()
    };

    for index in 0..locs_json.len() {
        let name = &locs_json[index].name;
        locs.push(name.clone());

        for way in &locs_json[index].adjacent_by_trackway {
            let other = &locs_json[*way].name;
            let (a, b) = (max(name, other), min(name, other));

            ways.insert(Way { side_a: a.clone(), side_b: b.clone(), way: WayType::Track });
        }

        for way in &locs_json[index].adjacent_by_waterway {
            let other = &locs_json[*way].name;
            let (a, b) = (max(name, other), min(name, other));

            ways.insert(Way { side_a: a.clone(), side_b: b.clone(), way: WayType::Water });
        }
    }

    println!("{locs:?}");
    println!("{ways:?}");
}