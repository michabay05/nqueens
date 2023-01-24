mod attack;
mod bb;
mod dna;
mod macros;
mod population;

use population::Population;

pub const POPULATION_COUNT: usize = 500;
pub const MUTATION_RATE: f32 = 0.03;

fn main() {
    let mut p = Population::new();
    p.init();

    let mut generation_count = 0;
    loop {
        p.calc_fitness();
        if p.check_all_state() {
            println!(
                "Found the right config after {} generations!!",
                generation_count
            );
            p.print_best();
            break;
        }
        println!("Generation {}", generation_count);
        p.print_best();
        println!("-----------------------------\n");
        p.generate(true);
        generation_count += 1;
    }
}
