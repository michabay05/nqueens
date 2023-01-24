use crate::attack::SlidingAttack;
use crate::bb::{BBTrait, BB};
use crate::dna::DNA;
use crate::GET_SQ;
use crate::{MUTATION_RATE, POPULATION_COUNT};

use rand::{thread_rng, Rng};

pub struct Population {
    // Store all the elements of the population
    // Each element is an instance of the 'DNA' struct
    elements: Vec<DNA>,

    // Store the mutation percentage
    mutate_rate: f32,

    // Store the precalculated moves for a queen
    // on all 64 squares
    attack_table: SlidingAttack,

    // Stores the mating pool and is initialized based on the fitness
    // of all the elements
    // The greater the fitness, the higher the element's chance of reproduction
    // This stores the indicies to element instead of the actual 'DNA' instance
    mating_pool: Vec<u16>,

    // Stores the index of the best element of the current population
    best: usize,
}

impl Population {
    // Constructor
    pub fn new() -> Self {
        Population {
            elements: Vec::with_capacity(POPULATION_COUNT),
            mutate_rate: MUTATION_RATE,
            attack_table: SlidingAttack::new(),
            mating_pool: Vec::new(),
            best: 0,
        }
    }

    pub fn init(&mut self) {
        // Initializes all the elements with randomized values
        for _ in 0..self.elements.capacity() {
            let mut new_dna = DNA::new();
            new_dna.randomize();
            // Add the new randomized 'DNA' to the elements list
            self.elements.push(new_dna);
        }
    }

    pub fn generate(&mut self, is_random: bool) {
        // Clear mating pool before regenerating
        self.mating_pool.clear();
        // Temporary, new elements list
        let mut new_elements: Vec<DNA> = Vec::new();

        // Based on the 'is_random' flag, the natural selection method
        // is selected
        if is_random {
            self.random_selection();
        } else {
            self.selective_selection();
        }

        // Update the elements list
        let mut rng = thread_rng();
        for i in 0..POPULATION_COUNT {
            // Random index from mating pool for two parents
            let rand_a: usize = rng.gen_range(0..self.mating_pool.len());
            let rand_b: usize = rng.gen_range(0..self.mating_pool.len());
            
            // Select parents based on the random indices
            let parent_a = &self.elements[self.mating_pool[rand_a] as usize];
            let parent_b = &self.elements[self.mating_pool[rand_b] as usize];

            // Crossover the genes of both parents to return a child
            let mut child = DNA::crossover(parent_a, parent_b);
            // Mutate the genes of the child based on the mutation rate to
            // continuously introduce variety
            DNA::mutate(&mut child, MUTATION_RATE);

            // Add the new child to the temporary element list
            new_elements.push(child);
        }

        // Update the existing elements list with the new elements list
        self.elements = new_elements;
    }

    fn random_selection(&mut self) {
        // Loop over every element and get their fitness and proportionally place
        // them in the mating pool to simulate higher or lower chance of reproduction
        for (i, el) in self.elements.iter().enumerate() {
            // Multiply the element's fitness by 100, set it to 'n', and add it
            // to the mating pool 'n' times
            let n = (el.get_fitness() * 100.0).floor() as u16;
            for _ in 0..n {
                self.mating_pool.push(i as u16);
            }
        }
    }

    fn selective_selection(&mut self) {
        todo!();
    }

    fn dna_to_bb(&self, dna_ref: &DNA) -> BB {
        // Converts the genes of a given dna into a bitboard representation
        let mut dna_bb: BB = 0;

        for i in 0..8u8 {
            let sq: u8 = GET_SQ!(i, dna_ref.get_gene_at(i as usize));
            dna_bb.set_bit(sq as i8);
        }
        dna_bb
    }

    pub fn check_all_state(&self) -> bool {
        // Checks all the elements of the population and return true if one 'DNA'
        // instance has the correct configuration

        for i in 0..self.elements.len() {
            // If current element has 8 queens correctly placed
            if self.count_matches(i) == 8 {
                return true;
            }
        }
        false
    }

    fn count_matches(&self, ind: usize) -> u8 {
        // Count the number of queens placed in a position in which
        // it can see another queen

        // Convert a given 'DNA' into a bitboard
        let mut dna_bb = self.dna_to_bb(&self.elements[ind]);
        // Stores the initial state of 'DNA's bitboard
        let init_bb = dna_bb;

        // Stores the numbers of matches found
        let mut matches: u8 = 0;

        // As long as there are queens left on the board which haven't
        // been checked
        while dna_bb != 0 {
            // Get the index of the queen and remove it from the bitboard
            let sq = dna_bb.pop_lsb();
            // Get all the possible moves of a queen from square 'sq'
            let all_moves = self.attack_table.bishop_moves[sq as usize]
                | self.attack_table.rook_moves[sq as usize];
            
            // If there is no overlap between the bitboards of possible moves
            // and the actual board, the current queen is correctly placed and
            // increment the match counter
            if init_bb & all_moves == 0 {
                matches += 1;
            }
        }
        matches
    }

    pub fn calc_fitness(&mut self) {
        // Update the fitness of all the population's element

        // Store the sum of all the fitness values and update the element's
        // number of matches
        let mut fitness_sum: u32 = 0;
        for i in 0..self.elements.len() {
            let match_count = self.count_matches(i);
            self.elements[i].set_matches(match_count);
            fitness_sum += match_count as u32;
        }

        // Normalize match and set to element's fitness
        for i in 0..self.elements.len() {
            let match_count = self.elements[i].get_matches();
            self.elements[i].set_fitness(match_count as f32 / fitness_sum as f32);
        }

        // Implement best(highest fitness) element
        for (i, el) in self.elements.iter().enumerate() {
            if el.get_fitness() > self.elements[self.best].get_fitness() {
                self.best = i;
            }
        }
    }

    /* CONVINIENCE METHODS FOR PRINTING VALUES */
    pub fn print_mating_pool(&self) {
        println!("Print mating pool");
        println!("{:?}", self.mating_pool);
        println!("\n");
    }
    pub fn print_mating_pool_count(&self) {
        println!("Print mating pool count");
        for (i, el) in self.elements.iter().enumerate() {
            let count = self
                .mating_pool
                .iter()
                .filter(|el| **el == (i as u16))
                .count();
            println!("{}: {}", i, count);
        }
    }

    pub fn print_elements(&self) {
        println!("Print elements");
        println!("{:#?}", self.elements);
        println!("\n");
    }

    pub fn print_fitness(&self) {
        println!("Fitness of every element");
        for (i, el) in self.elements.iter().enumerate() {
            println!("{}: {}", i, el.get_fitness());
        }
        println!("\n");
    }

    pub fn print_best(&self) {
        println!("Print best from population");
        let best_dna_ref = &self.elements[self.best];
        println!("Best element: {}", best_dna_ref.get_matches());
        BB::print(self.dna_to_bb(best_dna_ref));
        println!("\n");
    }
}
