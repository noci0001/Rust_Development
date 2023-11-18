#[derive(Debug)]
enum terrain {
    soil,
    lake,
    asphalt,
}

#[derive(Debug)]
enum vehicle_types {
    bycicle,
    SUV,
    hovercraft,
    None,
}

#[derive(Debug)]
struct Vehicle {
    name: String,
    vehicle_type: vehicle_types,
    top_speed: u32,
    battery_level: u32,
    weight: u32,
}

#[derive(Debug)]
struct Path {
    terrain_type: terrain,
    distance: u32,
}

fn drive_bycicle(vehicle: &Vehicle, total_distance: u32) {
    match total_distance {
        total_distance if (total_distance < 100) => {
            println!("Reachable");
        }
        _ => {
            println!("Absolutely not reachable");
        }
    }
}

fn drive_suv(vehicle: &Vehicle, total_distance: u32) {
    match total_distance {
        total_distance if (total_distance < 100) => {
            println!("Definitely Reachable");
        }
        total_distance if (total_distance < 200) => {
            println!("Reachable");
        }
        total_distance if (total_distance < 400) => {
            println!("Reachable, but the travel will be long");
        }
        _ => {
            println!("Distance out of parsable range");
            std::process::exit(1);
        }
    }
}

fn drive_hovercraft(vehicle: &Vehicle, total_distance: u32) {
    match total_distance {
        total_distance if (total_distance < 100) => {
            println!("Reachable");
        }
        total_distance if (total_distance < 200) => {
            println!("Reachable");
        }
        total_distance if (total_distance < 400) => {
            println!("Reachable, pretty doable");
        }
        _ => {
            println!("Absolutely not reachable");
        }
    }
}

fn drive(vehicle: &Vehicle, total_distance: u32) -> u32 {
    match &vehicle.vehicle_type {
        vehicle_types::bycicle => {
            println!("Bycicling...");
            drive_bycicle(&vehicle, total_distance);
        }
        vehicle_types::SUV => {
            println!("SUVing...");
            drive_suv(&vehicle, total_distance);
        }
        vehicle_types::hovercraft => {
            println!("Hovercrafting...");
            drive_hovercraft(&vehicle, total_distance);
        }
        _ => {
            println!("Vehicle not recognized, aborting...");
            std::process::exit(1);
        }
    }
    total_distance
}

fn main() {

    let hovercraft = Vehicle {
        name: String::from("HC190"),
        vehicle_type: vehicle_types::hovercraft,
        top_speed: 190,
        battery_level: 100,
        weight: 5000,
    };

    let suv = Vehicle {
        name: String::from("SUV160"),
        vehicle_type: vehicle_types::SUV,
        top_speed: 160,
        battery_level: 100,
        weight: 5000,
    };

    let bycicle = Vehicle {
        name: String::from("BCY25"),
        vehicle_type: vehicle_types::bycicle,
        top_speed: 25,
        battery_level: 100,
        weight: 22,
    };

    let vehicles: Vec<Vehicle> = vec![hovercraft, suv, bycicle];

    let mut complete_path: Vec<Path> = Vec::new();

    complete_path.push( 
        Path {
            terrain_type: terrain::soil,
            distance: 10,
        });
    
    complete_path.push( 
        Path {
            terrain_type: terrain::asphalt,
            distance: 36,
        });

    complete_path.push( 
        Path {
            terrain_type: terrain::lake,
            distance: 289,
        });

    println!("Vehicles available: \n");

    let mut total_distance: u32 = 0;
    for vehicle in &vehicles {
        println!("Name: {}\nVehicle type: {:?}\n\n", vehicle.name, vehicle.vehicle_type);
    }

    println!("Info path ahead: ");
    for path in complete_path {
        println!("Will find {} kilometers of type {:?}\n", path.distance, path.terrain_type);
        total_distance += path.distance;
    }

    println!("Total distance is: {}", total_distance);

    for vehicle in &vehicles {
        drive(vehicle, total_distance);
    }
}
