use rand::distributions::{Uniform, Exp, Alphanumeric};
use rand::prelude::*;

#[derive(Default)]
pub struct UserData {
    name: String,
    username: String,
    age: String,
    email: String,
    bio: String,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn evo_algo() {
        let mut rng = thread_rng();
        let mut rng2 = rand::rngs::EntropyRng::new();
        let target: String = rand_string(&mut rng, 10);
        let start: String = rand_string(&mut rng2, 10);
        let init_fitness = fitness(target, start);
        println!("START: [Fitness {}] Start {}, Target {}", init_fitness, start, target);
        for i in 1..100 {
            let new = mutate(rng, start, 0.05);
            let fitness = fitness(target, new);
            println!("Gen {}: [Fitness {}] Mutation {}, Target {}", init_fitness, new, target);
        }
        let new = mutate(rng, target, 0.05);
        let fitness = fitness(target, new);
        println!("END: [Fitness {}] Start {}, Target {}", fitness, start, target);
    }

}

impl UserData {
    pub fn new() -> Self { UserData::default() }
}

fn rand_string<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut out = String::with_capacity(len);
    let rn: Vec<char> = rng.sample_iter(Alphanumeric).take(len).collect();
    out.extend(rn);
    out
}

fn mutate<R: Rng>(rng: &mut R, string: &str, mut_rate: f64) -> String {
    let maybe_mutate = |c: char| {
        if rng.gen_bool(mut_rate) {
            rand_string(rng, 1).remove(0)
        } else {
            c
        }
    };
    string.chars().map(maybe_mutate).collect()
}

fn fitness(target: String, sentence: String) -> usize {
    sentence
        .chars()
        .zip(target.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count()
}
