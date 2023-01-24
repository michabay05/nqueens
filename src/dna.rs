use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug)]
pub struct DNA {
    genes: [u8; 8],
    matches: u8,
    fitness: f32,
}

impl DNA {
    pub fn new() -> Self {
        DNA {
            genes: [0; 8],
            matches: 0,
            fitness: 0.0,
        }
    }

    pub fn randomized_new() -> Self {
        let mut dna_instance = DNA {
            genes: [0; 8],
            matches: 0,
            fitness: 0.0,
        };
        dna_instance.randomize();
        dna_instance
    }

    pub fn randomize(&mut self) {
        let mut rng = thread_rng();
        for i in 0..8 {
            self.genes[i] = rng.gen_range(0..8);
        }
    }

    // GET METHODS
    pub fn get_gene_at(&self, ind: usize) -> u8 {
        self.genes[ind]
    }

    pub fn set_gene_at(&mut self, ind: usize, val: u8) {
        self.genes[ind] = val;
    }

    pub fn get_fitness(&self) -> f32 {
        self.fitness
    }

    pub fn set_fitness(&mut self, new_fitness: f32) {
        self.fitness = new_fitness;
    }

    pub fn get_matches(&self) -> u8 {
        self.matches
    }

    pub fn set_matches(&mut self, new_match: u8) {
        self.matches = new_match;
    }

    pub fn print_genes(&self) {
        print!("[");
        for i in 0..8 {
            print!("{}, ", self.genes[i]);
        }
        println!("]");
    }

    /* ASSOCIATED FUNCTIONS */
    pub fn crossover(parent_a: &Self, parent_b: &Self) -> Self {
        let mut child_dna = DNA::new();

        // Random split point
        let mut rng = thread_rng();
        let random_split_point = rng.gen_range(0..(8 - 1));
        for i in 0..8 {
            if i < random_split_point {
                // Parent A genes
                child_dna.set_gene_at(i, parent_a.get_gene_at(i));
            } else {
                // Parent B genes
                child_dna.set_gene_at(i, parent_b.get_gene_at(i));
            }
        }
        child_dna
    }

    pub fn mutate(dna_el: &mut DNA, mutate_rate: f32) {
        let mut rng = thread_rng();
        for i in 0..8 {
            let rand = rng.gen::<f32>();
            if rand < mutate_rate {
                let new_random_el = rng.gen_range(0..8);
                dna_el.set_gene_at(i, new_random_el);
            }
        }
    }
}
