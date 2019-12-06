use std::collections::HashMap;

fn parse_orbits(input: &str) -> Vec<(String, String)> {

    let mut pairs = vec![];

    for line in input.lines() {
        let objects: Vec<_> = line.split(")")
            .collect();
        pairs.push((objects[0].to_string(), objects[1].to_string()));
    }

    pairs
}

fn build_direct_mapping(pairs: Vec<(String, String)>) -> HashMap<String, Vec<String>> {

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (center, sat) in pairs {
        if let Some(sats) = map.get_mut(&center) {
            sats.push(sat);
        } else {
            map.insert(center, vec![sat]);
        }
    }

    map
}

fn populate_indirects_for_object(
    map: &HashMap<String, Vec<String>>,
    new_map: &mut HashMap<String, (Vec<String>, Vec<String>)>,
    object: &str
) {

    // println!("populating indirects for {}", object);

    if new_map.contains_key(object) {
        return;
    }

    // println!("map: {:?}", map);
    let directs = map.get(object)
        .map(|d| d.clone())
        .unwrap_or(vec![]);
    let mut indirects = vec![];

    for sat in &directs {

        populate_indirects_for_object(map, new_map, sat);

        let mut sat_directs = new_map[sat].0.clone();
        indirects.append(&mut sat_directs);

        let mut sat_indirects = new_map[sat].1.clone();
        indirects.append(&mut sat_indirects);
    }

    new_map.insert(object.to_string(), (directs, indirects));
}



fn build_indirect_mapping(map: HashMap<String, Vec<String>>) -> HashMap<String, (Vec<String>, Vec<String>)> {

    let mut new_map: HashMap<String, (Vec<String>, Vec<String>)> = HashMap::new();

    populate_indirects_for_object(&map, &mut new_map, "COM");

    new_map
}


const ORBITS: &'static str = include_str!("orbits.txt");



pub fn part1() {
//     let pairs = parse_orbits("COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L");

    let pairs = parse_orbits(ORBITS);

    let map = build_direct_mapping(pairs);

    let map = build_indirect_mapping(map);

    // println!("{:#?}", map);

    let mut total = 0;

    for (_, (dir, ind)) in map {
        total += dir.len();
        total += ind.len();
    }

    println!("{}", total);
}

type FullMap = HashMap<String, (Vec<String>, Vec<String>)>;

fn comm_inner(map: &FullMap, cur: &str, obj_a: &str, obj_b: &str) -> String {

    for sat in &map[cur].0 {

        let (dir, ind) = &map[sat];

        let has_a = dir.iter().any(|o| o == obj_a) || ind.iter().any(|o| o == obj_a);
        let has_b = dir.iter().any(|o| o == obj_b) || ind.iter().any(|o| o == obj_b);

        if has_a && has_b {
            return comm_inner(map, sat, obj_a, obj_b);
        }
    }

    cur.to_string()
}

/// Finds the outermost common object between the two objects
fn comm(map: &FullMap, obj_a: &str, obj_b: &str) -> String {

    comm_inner(map, "COM", obj_a, obj_b)
}


/// Computes distance between src and dst.
///
/// dst MUST directly or indirectly orbit src
fn dist(map: &FullMap, src: &str, dst: &str) -> usize {

    let (dir, ind) = &map[src];

    // println!("{} is orbiting {}", dst, src);
    if dir.iter().any(|o| o == dst) {
        // println!("fount {} orbiting {}", dst, src);
        return 0;
    }

    for sat in dir {

        let (dir, ind) = &map[sat];

        let has_dst = dir.iter().any(|o| o == dst) || ind.iter().any(|o| o == dst);

        if has_dst {
            // println!("recursing to {} for {}", sat, dst);
            return 1 + dist(map, sat, dst);
        }
    }

    panic!("{} does not orbit {}", dst, src);
}




pub fn part2() {

    let pairs = parse_orbits(ORBITS);

//     let pairs = parse_orbits("COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
// K)YOU
// I)SAN");

    let map = build_direct_mapping(pairs);

    let map = build_indirect_mapping(map);

    let comm = comm(&map, "YOU", "SAN");

    println!("comm: {}", comm);

    let dist_you = dist(&map, &comm, "YOU");
    println!("dist_you: {}", dist_you);

    let dist_san = dist(&map, &comm, "SAN");
    println!("dist_san: {}", dist_san);

    println!("{}", dist_you + dist_san);
}
