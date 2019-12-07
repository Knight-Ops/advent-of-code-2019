use std::collections::{HashMap, HashSet, hash_map::Entry};
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Planet<'a> {
    id: usize,
    orbits: Option<&'a str>,
    orbiting_planets: Vec<&'a str>,
    indirect_orbits: usize,
}

impl<'a> Planet<'a> {
    pub fn new(id: usize, orbits: Option<&'a str>) -> Planet<'a> {
        Planet {
            id,
            orbits,
            orbiting_planets: vec![],
            indirect_orbits: 0,
        }
    }

    fn add_orbiting_planet(&mut self, orbiting_planet: &'a str) {
        self.orbiting_planets.push(orbiting_planet);
    }

    fn set_indirect_orbits(&mut self, value: usize) {
        self.indirect_orbits = value;
    }

    fn add_orbit(&mut self, orbit: Option<&'a str>) {
        self.orbits = orbit;
    }

    // pub fn get_orbiting_planets(&self) -> Option<Vec<&str>> {
    //     self.orbiting_planets.unwrap
    // }
}

#[derive(Debug, Clone)]
struct Orbit {
    reference: String,
    satellite: String,
}

impl Orbit {
    pub fn new(input: &str) -> Orbit {
        let split: Vec<&str> = input.split(")").collect();

        Orbit {
            reference: split[0].to_string(),
            satellite: split[1].to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Universe<'a> {
    planets: HashMap<&'a str, Planet<'a>>,
    lanes : HashMap<usize, usize>,
}

impl<'a> Universe<'a> {
    pub fn new(input: &'a Vec<Orbit>) -> Universe<'a> {
        let mut universe = Universe {
            planets: HashMap::new(),
            lanes: HashMap::new()
        };

        let mut planet_id = 0;

        universe.planets.insert("COM", Planet::new(planet_id, None));
        planet_id += 1;

        input.iter().for_each(|x| {
            let mut satellite_id = 0;
            let mut reference_id = 0;
            match universe
                .planets
                .entry(&x.reference) {
                    Entry::Occupied(mut occupied) => {
                        occupied.get_mut().add_orbiting_planet(&x.satellite);
                        reference_id = occupied.get().id;
                    },
                    Entry::Vacant(vacant) => {
                        vacant.insert(Planet::new(planet_id, None)).add_orbiting_planet(&x.satellite);
                        reference_id = planet_id;
                        planet_id += 1;
                    },
                };

            match universe
                .planets
                .entry(&x.satellite) {
                    Entry::Occupied(mut occupied) => {
                        occupied.get_mut().add_orbit(Some(&x.reference));
                        satellite_id = occupied.get().id
                    },
                    Entry::Vacant(vacant) => {
                        vacant.insert(Planet::new(planet_id, Some(&x.reference))).add_orbit(Some(&x.reference));
                        satellite_id = planet_id;
                        planet_id += 1;
                    },
                }

                universe.lanes.insert(satellite_id, reference_id);

        });

        universe
    }

    fn set_indirect_orbits(&mut self, input: &str, depth: usize) {
        {
            let planet = self.planets.get_mut(input).unwrap();
            planet.set_indirect_orbits(depth);
        }
        let planet = self.planets.get(input).unwrap();
        planet.orbiting_planets.clone().iter().for_each(|x| {
            self.set_indirect_orbits(x, if input == "COM" { depth } else { depth + 1 })
        });
    }

    /// This finds paths between two terminal objects and will not work on non-terminal (has orbiting planets)
    pub fn find_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let start = start.to_string();
        let end = end.to_string();
        let visited_planets = vec![start.clone()];
        let planet_name = self
            .planets
            .get(start.as_str())
            .unwrap()
            .orbits
            .unwrap()
            .to_string();
        debug_print!("Starting planet is : {}", planet_name);

        self.explore(planet_name, &visited_planets, end)
    }

    fn explore(
        &self,
        planet_name: String,
        visited_planets: &[String],
        end: String,
    ) -> Option<Vec<String>> {
        if planet_name == end {
            let mut vec = visited_planets.to_vec();
            vec.push(end);
            return Some(vec);
        }
        let mut past_planets: HashSet<String> = HashSet::new();
        visited_planets.to_vec().iter().for_each(|x| {
            past_planets.insert((*x).clone());
        });

        let planet = self.planets.get(planet_name.as_str()).unwrap();
        debug_print!(
            "Planet : {} - {:?} - {:?}",
            planet.id,
            planet.orbits,
            planet.orbiting_planets
        );

        let mut possible_visit_locations: HashSet<String> = HashSet::new();
        if let Some(val) = planet.orbits {
            possible_visit_locations.insert(val.to_string());
        }
        planet.orbiting_planets.iter().for_each(|x| {
            possible_visit_locations.insert(x.to_string());
        });
        let visit_locations: HashSet<_> =
            possible_visit_locations.difference(&past_planets).collect();
        debug_print!("Locations to visit : {:?}", visit_locations);

        let mut visited_planets = visited_planets.to_vec();
        visited_planets.push(planet_name);
        let paths: Vec<_> = visit_locations
            .par_iter()
            .map(|x| self.explore(x.to_string(), &visited_planets, end.clone()))
            .filter(|x| x.is_some())
            .collect();

        if paths.is_empty() {
            None
        } else {
            paths[0].to_owned()
        }
    }

    fn intersect(&self, start: &str, end: &str) -> usize {
        let from_start = self.traverse_to_center(start);
        let from_end = self.traverse_to_center(end);

        let intersection : Vec<_> = from_end.iter().filter(|x| from_start.contains(x)).collect();

        let start_to_intersect : Vec<_> = from_start.iter().filter(|x| !intersection.contains(x)).collect();
        let intersect_to_end : Vec<_> = from_end.iter().filter(|x| !intersection.contains(x)).collect();

        debug_print!("Intersection : {:?}", intersection);
        start_to_intersect.len() + intersect_to_end.len()
    }

    fn traverse_to_center(&self, from: &str) -> Vec<usize> {
        let mut reference_id = self.planets.get(from).unwrap().id;
        let mut path = vec![];
        while reference_id != 0 {
            reference_id = self.lanes[&reference_id];
            path.push(reference_id);
        }

        path
    }

    fn intersect_hashmap(&self, start: &str, end: &str) -> usize {
        let mut from_start = self.traverse_to_center_hashmap(start);
        
        let mut reference_id = self.planets.get(end).unwrap().id;
        let mut path_len = 0;
        while reference_id != 0 {
            reference_id = self.lanes[&reference_id];
            match from_start.entry(reference_id) {
                Entry::Occupied(occupied) => {
                    return *occupied.get() + path_len;
                },
                Entry::Vacant(_) => {
                    path_len += 1;
                }
            }
        }

        unreachable!()
    }

    fn traverse_to_center_hashmap(&self, from: &str) -> HashMap<usize, usize> {
        let mut hm = HashMap::new();
        let mut reference_id = self.planets.get(from).unwrap().id;
        let mut path_len = 0;
        while reference_id != 0 {
            reference_id = self.lanes[&reference_id];
            hm.insert(reference_id, path_len);
            path_len += 1;
        }

        hm
    }
}

#[aoc_generator(day6)]
fn process_input(input: &str) -> Vec<Orbit> {
    input.trim().lines().map(|x| Orbit::new(x)).collect()
}

#[aoc(day6, part1)]
fn d6p1(input: &Vec<Orbit>) -> usize {
    let direct_orbits = input.len();
    debug_print!("Direct Orbits : {}", direct_orbits);

    let mut universe = Universe::new(input);
    universe.set_indirect_orbits("COM", 0);
    debug_print!("Universe : {:#?}", universe);

    let indirect_orbits: usize = universe
        .planets
        .iter()
        .map(|(_id, planet)| planet.indirect_orbits)
        .sum();

    direct_orbits + indirect_orbits
}

#[aoc(day6, part2)]
fn d6p2(input: &Vec<Orbit>) -> usize {
    debug_print!("Orbits : {:#?}", input);
    let universe = Universe::new(input);
    debug_print!("Universe : {:#?}", universe);

    let path = universe.find_path("YOU", "SAN");

    path.unwrap().len() - 3
}

#[aoc(day6, part2, intersect)]
fn d6p2_intersect(input: &Vec<Orbit>) -> usize {
    debug_print!("Orbits : {:#?}", input);
    let universe = Universe::new(input);
    debug_print!("Universe : {:#?}", universe);

    universe.intersect("YOU", "SAN")
}

#[aoc(day6, part2, intersect_hashmap)]
fn d6p2_intersect_hashmap(input: &Vec<Orbit>) -> usize {
    debug_print!("Orbits : {:#?}", input);
    let universe = Universe::new(input);
    debug_print!("Universe : {:#?}", universe);

    universe.intersect_hashmap("YOU", "SAN")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let orbits = process_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");

        assert_eq!(d6p1(&orbits), 42);
    }

    #[test]
    fn test2() {
        let orbits =
            process_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");

        assert_eq!(d6p2(&orbits), 4);
    }
}
