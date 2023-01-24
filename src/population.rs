use crate::attack::SlidingAttack;
use crate::bb::{BBTrait, BB};
use crate::dna::DNA;
use crate::GET_SQ;
use crate::{MUTATION_RATE, POPULATION_COUNT};

use rand::{thread_rng, Rng};

pub struct Population {
    elements: Vec<DNA>,
    mutate_rate: f32,
    attack_table: SlidingAttack,
    mating_pool: Vec<u16>,
    best: usize,
}

impl Population {
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
        for _ in 0..self.elements.capacity() {
            let mut new_dna = DNA::new();
            new_dna.randomize();
            self.elements.push(new_dna);
        }
    }

    pub fn generate(&mut self, is_random: bool) {
        self.mating_pool.clear();
        let mut new_elements: Vec<DNA> = Vec::new();
        if is_random {
            self.random_selection();
        } else {
            self.selective_selection();
        }

        // Update the elements list
        let mut rng = thread_rng();
        for i in 0..POPULATION_COUNT {
            let rand_a: usize = rng.gen_range(0..self.mating_pool.len());
            let rand_b: usize = rng.gen_range(0..self.mating_pool.len());
            let parent_a = &self.elements[self.mating_pool[rand_a] as usize];
            let parent_b = &self.elements[self.mating_pool[rand_b] as usize];

            let mut child = DNA::crossover(parent_a, parent_b);
            DNA::mutate(&mut child, MUTATION_RATE);

            new_elements.push(child);
        }

        self.elements = new_elements;
    }

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

    fn random_selection(&mut self) {
        // Loop over every element and get their fitness
        // and proportionally place them in the mating pool
        for (i, el) in self.elements.iter().enumerate() {
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
        let mut dna_bb: BB = 0;

        for i in 0..8u8 {
            let sq: u8 = GET_SQ!(i, dna_ref.get_gene_at(i as usize));
            dna_bb.set_bit(sq as i8);
        }
        dna_bb
    }

    pub fn check_all_state(&self) -> bool {
        let mut succeeded = false;
        for i in 0..self.elements.len() {
            if self.count_matches(i) == 8 {
                succeeded = true;
            }
        }
        succeeded
    }

    fn count_matches(&self, ind: usize) -> u8 {
        let mut dna_bb = self.dna_to_bb(&self.elements[ind]);
        let init_bb = dna_bb;

        let mut matches: u8 = 0;

        while dna_bb != 0 {
            let sq = dna_bb.pop_lsb();
            let all_moves = self.attack_table.bishop_occupancy[sq as usize]
                | self.attack_table.rook_occupancy[sq as usize];
            if init_bb & all_moves == 0 {
                matches += 1;
            }
        }
        matches
    }

    pub fn calc_fitness(&mut self) {
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
}
