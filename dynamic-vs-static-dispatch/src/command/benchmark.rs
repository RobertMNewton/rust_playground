use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

#[derive(Clone)]
pub enum Commands {
    Half,
    Double,
    Increment,
    Decrement,
    Square,
    Root,
}

impl Distribution<Commands> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Commands {
            match rng.gen_range(0..6) {
                0 => Commands::Half,
                1 => Commands::Double,
                2 => Commands::Increment,
                3 => Commands::Decrement,
                4 => Commands::Square,
                _ => Commands::Root
            }
    }
}

pub struct Benchmark {
    pub command_list: Vec<Commands>,
    pub input: f64,
    pub output: f64,
}

impl Benchmark {
    pub fn generate_random(size: usize) -> Benchmark {
        let mut rng = rand::thread_rng();
        
        let (input, command_list): (f64, Vec<Commands>) = (rng.sample(Uniform::new(-1.0, 1.0)), (0..size).map(|_| rng.gen()).collect());
        
        let mut output = input;
        command_list.iter().for_each(|command| {
            match command {
                Commands::Half => output /= 2.0,
                Commands::Double => output *= 2.0,
                Commands::Increment => output += 1.0,
                Commands::Decrement => output -= 1.0,
                Commands::Square => output = output.powf(2.0),
                Commands::Root => output = output.powf(0.5),
            }
        });

        Benchmark {
            command_list: command_list,
            input: input,
            output: output,
        }
    }
}

pub trait Solution {
    fn new(commands: Vec<Commands>) -> Self;
    fn run(&self, input: f64) -> f64;
}