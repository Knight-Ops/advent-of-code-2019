use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Planet<'a> {
    id: &'a str,
    orbits: Option<&'a str>,
    orbiting_planets: Vec<&'a str>,
    indirect_orbits: usize,
}

impl<'a> Planet<'a> {
    pub fn new(id: &'a str, orbits: Option<&'a str>) -> Planet<'a> {
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
}

impl<'a> Universe<'a> {
    pub fn new(input: &'a Vec<Orbit>) -> Universe<'a> {
        let mut universe = Universe {
            planets: HashMap::new(),
        };

        universe.planets.insert("COM", Planet::new("COM", None));

        input.iter().for_each(|x| {
            universe
                .planets
                .entry(&x.reference)
                .or_insert(Planet::new(&x.reference, None))
                .add_orbiting_planet(&x.satellite);

            universe
                .planets
                .entry(&x.satellite)
                .or_insert(Planet::new(&x.satellite, Some(&x.reference)));
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

        // We need to mark where we have been so we don't loop
        // visited_planets.push(planet_name.clone());

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
        // debug_print!("Got planet : {:#?}", planet);
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
            .iter()
            .map(|x| self.explore(x.to_string(), &visited_planets, end.clone()))
            .filter(|x| x.is_some())
            .collect();

        if paths.is_empty() {
            None
        } else {
            paths[0].to_owned()
        }
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

    let path = universe.find_path("YOU", "SAN");

    path.unwrap().len() - 3
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let orbits = process_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        //println!("{:#?}", orbits);
        assert_eq!(d6p1(&orbits), 42);
    }

    #[test]
    fn test2() {
        let orbits =
            process_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");

        assert_eq!(d6p2(&orbits), 5);
    }
}
