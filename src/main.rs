use wasm_game_of_life::Universe;

fn main() {
    let mut universe: Universe = Universe::new(10, 10);
    println!("{}", universe);

    for i in 0..100 {
        universe.tick();
    }
    println!("{}", universe);


    let rand_universe: Universe = Universe::rand(10, 10);
    println!("{}", rand_universe);

}
