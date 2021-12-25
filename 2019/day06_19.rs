///--- Day 6: Universal Orbit Map ---
///
///You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
///
///Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
///
///                  \
///                   \
///                    |
///                    |
///AAA--> o            o <--BBB
///                    |
///                    |
///                   /
///                  /
///
///In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
///
///Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
///
///Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
///
///For example, suppose you have the following map:
///
///COM)B
///B)C
///C)D
///D)E
///E)F
///B)G
///G)H
///D)I
///E)J
///J)K
///K)L
///
///Visually, the above map of orbits looks like this:
///
///        G - H       J - K - L
///       /           /
///COM - B - C - D - E - F
///               \
///                I
///
///In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
///
///Here, we can count the total number of orbits as follows:
///
///    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
///    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
///    COM orbits nothing.
///
///The total number of direct and indirect orbits in this example is 42.
///
///What is the total number of direct and indirect orbits in your map data?
///
///Your puzzle answer was 278744.
///--- Part Two ---
///
///Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
///
///You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
///
///For example, suppose you have the following map:
///
///COM)B
///B)C
///C)D
///D)E
///E)F
///B)G
///G)H
///D)I
///E)J
///J)K
///K)L
///K)YOU
///I)SAN
///
///Visually, the above map of orbits looks like this:
///
///                          YOU
///                         /
///        G - H       J - K - L
///       /           /
///COM - B - C - D - E - F
///               \
///                I - SAN
///
///In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
///
///    K to J
///    J to E
///    E to D
///    D to I
///
///Afterward, the map of orbits looks like this:
///
///        G - H       J - K - L
///       /           /
///COM - B - C - D - E - F
///               \
///                I - SAN
///                 \
///                  YOU
///
///What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)
///
///Your puzzle answer was 475.
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    solvep1();
    solvep2();
}

#[allow(dead_code)]
pub fn solvep1() -> usize {
    let file = File::open("day06_19").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Couldn't read I guess");
    let mut cur_planet_layer:Vec<&str> = vec!["COM"];
    let mut next_planet_layer:Vec<&str> = Vec::new();
    let mut counter = 0;
    let mut val = 0;

    loop {
        counter += 1;
        for line in contents.split("\n") {
            let mut planet_iterator = line.split(")");
            let planet1 = planet_iterator.next();
            if let None = planet1 {break;}
            let planet2 = planet_iterator.next();
            if let None = planet2 {break;}
            if cur_planet_layer.contains(&planet1.unwrap()) {
                val += counter;
                next_planet_layer.push(&planet2.unwrap());
            }
        }
        println!("layer {:?}", cur_planet_layer);
        if next_planet_layer.is_empty() {break;}
        cur_planet_layer = next_planet_layer.clone();
        next_planet_layer.clear();
    }
    println!("{}", val);
    val
}

#[allow(dead_code)]
pub fn solvep2() -> usize {
    let file = File::open("../resources/day6").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Couldn't read I guess");
    let mut santa = "SAN";
    let mut you = "YOU";
    let mut you_path = vec!("YOU");
    let mut santa_path = vec!("SAN");
    loop {
        for line in contents.split("\n") {
            let mut planet_iterator = line.split(")");
            let planet1 = planet_iterator.next();
            if let None = planet1 {break;}
            let planet2 = planet_iterator.next();
            if let None = planet2 {break;}

            if planet2.unwrap() == you.to_string() && planet2.unwrap() != "QFR" {
                you = planet1.unwrap();
                you_path.push(&you);
            }
            if planet2.unwrap() == santa.to_string() {
                santa = planet1.unwrap();
                santa_path.push(&santa);
            }
        }
        println!("Santa: {}, You:{}", santa, you);
        if you_path.contains(&santa) {break;}
    }
    //let index = you_path.iter().position(|&r| r == santa);
    //println!("{}", index.unwrap());
    //println!("{}", val);
    you_path.remove(0);
    santa_path.remove(0);
    print!("you_path: ");
    for i in &you_path {print!("{}-", i);}
    println!("");
    println!("{}", you_path.len());
    print!("santa_path: ");
    for i in &santa_path {print!("{}-", i);}
    println!("");
    println!("{}", santa_path.len());
    println!("final ans: {}", santa_path.len() + you_path.len() - 2);
    santa_path.len() + you_path.len() - 2
}
/*
This code is also too slow to run
pub fn solvep1() -> std::io::Result<()> {
    let file = File::open("../resources/day6p1").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Couldn't read I guess");
    let mut data:HashMap<&str, Vec<&str>> = HashMap::new();
    data.insert("COM", Vec::new());
    let mut counter = 0;

    'outer: loop {
        'file_iterator: for line in contents.split("\n") {
            let mut planet_iterator = line.split(")");
            let planet1 = planet_iterator.next();
            if let None = planet1 {break 'outer;}
            let planet2 = planet_iterator.next();
            if let None = planet2 {break 'outer;}

            match data.get_mut(&planet1.unwrap()) {
                Some(i) => {
                    i.push(&planet2.unwrap());
                    data.insert(&planet2.unwrap(), Vec::new());
                    println!("{:?}", data);
                    counter = counter + 1;
                    if counter >= 3 {break 'outer};
                    //break 'outer;
                    //break 'file_iterator;
                },
                None => continue,
            };
        }
        println!("break file_iterator {:?}", data);
    }
    // println!("{}", contents);
    // let mut data:HashMap<String, usize> = HashMap::new();

    Ok(())
}
*/
/*
This code is too slow to run
pub fn solvep1() -> std::io::Result<()> {
    let file = File::open("../resources/day6p1").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Couldn't read I guess");
    let mut data:Vec<Vec<&str>> = Vec::new();
    data.push(vec!("COM"));
    let mut is_finished = true;

    while is_finished {
        'file_iter: for line in contents.split("\n") {
            let mut planet_iterator = line.split(")");

            if let None = planet1 {is_finished = false; break;}
            let planet2 = planet_iterator.next();
            if let None = planet2 {is_finished = false; break;}

            for orbit in 0..data.len() {
                if data[orbit].contains(&planet1.unwrap()) {
                    if orbit + 1 == data.len() {
                        data.push(vec!(&planet2.unwrap()));
                    }
                    data[orbit + 1].push(&planet2.unwrap());
                    break 'file_iter;
                }
            }
        }
    }
    Ok(())
}
*/
