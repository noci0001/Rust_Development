use std::collections::HashSet;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {

       let mut sam = HashSet::new();
       let mut rafa = HashSet::new();

       sam.insert("Shrek".to_string());
       sam.insert("Mulan".to_string());
       sam.insert("Encanto".to_string());
       sam.insert("Tarzan".to_string());
       sam.insert("Up".to_string());

       rafa.insert("Bug's life".to_string());
       rafa.insert("Mulan".to_string());
       rafa.insert("Monsters Incl.".to_string());
       rafa.insert("Tarzan".to_string());
       rafa.insert("Finding nemo".to_string());

       println!("\nMovies that Sam likes:\n");
       for movie in &sam {
            println!("{}", movie);
       }
       println!("\nMovies that Rafa likes:\n");
       for movie in &rafa {
            println!("{}", movie);
       }

       let union = &sam & &rafa;

       println!("\nMovies we both like: \n");

       for movies in &union {
            println!("{:?}", movies);
       }

       //create randomizer
       let mut rng = thread_rng();

       
       //pick random movie from union
    let random_movie = union.choose(&mut rng).unwrap();
       println!("Movie picker has chosen: {:?}", random_movie.);
}