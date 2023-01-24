mod attack;
mod bb;
mod dna;
mod macros;
mod population;

/*
    **NOTE**: This program only works for an n by n board with n queens
              where n == 8; Support for other values of n are potentially
              coming
*/

use population::Population;

// Number of element in a population set
pub const POPULATION_COUNT: usize = 500;
// Mutation percentage: 3 percent
pub const MUTATION_RATE: f32 = 0.03;

fn main() {
    // Create and initialize a population set
    let mut p = Population::new();
    p.init();

    // Generation counter
    let mut generation_count = 0;
    loop {
        // Calcuate the fitness of all the elements in the population
        p.calc_fitness();

        // If one element succeeds, print the element that succeed and break
        if p.check_all_state() {
            println!(
                "Found the right config after {} generations!!",
                generation_count
            );
            p.print_best();
            break;
        }
        // Print the generation number alongside the best for the current generation
        println!("Generation {}", generation_count);
        p.print_best();
        println!("-----------------------------\n");

        // Reproduce(Regenerate) the elements in the population
        p.generate(true);

        // Increment population count
        generation_count += 1;
    }
}
