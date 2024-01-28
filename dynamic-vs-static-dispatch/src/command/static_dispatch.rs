use std::process::Command;

use super::benchmark::{Solution, Commands};

trait StaticCommand {
    fn operate(&self, input: f64) -> f64;
}

macro_rules! impl_static_command {
    ($name:ident, $op:expr) => {
        pub struct $name<N, T: StaticCommand> {
            next: Option<Box<T>>,
        }

        impl<N, T: StaticCommand> $name<N, T> {
            pub fn new(next: Option<Box<T>>) -> Self {
                Self { next }
            }
        }

        impl<N, T: StaticCommand> StaticCommand for $name<N, T> {
            fn operate(&self, x: f64) -> f64 {
                match &self.next {
                    Some(command) => command.operate($op(x)),
                    None => $op(x),
                }
            }
        }
    };
}

impl_static_command!(Half, |x| x / 2.0);
impl_static_command!(Double, |x| x * 2.0);
impl_static_command!(Increment, |x| x + 1.0);
impl_static_command!(Decrement, |x| x - 1.0);
impl_static_command!(Square, |x: f64| x.powf(2.0));
impl_static_command!(Root, |x: f64| x.powf(0.5));

pub struct StaticSolution<T: StaticCommand> {
    head: Box<T>,
}

impl <T: StaticCommand>Solution for StaticSolution<T> {
    fn new(commands: Vec<Commands>) -> Self {
        let mut head: StaticCommand;

        for command in commands {

        }
         // not sure about this part

        StaticSolution { 
            head: head,
        }
    }

    fn run(&self, input: f64) -> f64 {
        self.head.operate(input)
    }
}