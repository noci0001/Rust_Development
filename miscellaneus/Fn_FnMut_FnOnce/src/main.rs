#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

// fn sort_pop(city: &mut Vec<City>) {
//     city.sort_by_key(pop_helper)
// }

// //basically get_population
// fn pop_helper(pop: &City) -> u64 {
//     pop.population
// }

fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population)
}

fn main() {
    let a = City{ city: String::from("A"), population: 100};
    let b = City{ city: String::from("B"), population: 57};
    let c = City{ city: String::from("C"), population: 140};
    let d = City{ city: String::from("D"), population: 70};

    let mut vec: Vec<City> = Vec::new();

    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);

    // sort_pop(&mut vec);
    sort_pop_closure(&mut vec);
    println!("{:?}", vec);

    let add = |x: i32| ->i32 {x + 1};
    let add_v2 = |x| x+1;
    add_v2(1);

    let example = |x| x;

    //TYPE IS INFERRED BY THE FIRST INSTANCE UTILIZED, IN THIS CASE STRING
    let string = example(String::from("string"));

    //Fn, FnMut, FnOnce
}
