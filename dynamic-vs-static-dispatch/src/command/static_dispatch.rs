use std::{borrow::Borrow, process::Command};

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

pub struct StaticSolution {
    head: Box<dyn StaticCommand>,
}


impl Solution for StaticSolution {
    fn new(commands: Vec<Commands>) -> Self {
        let mut curr: Box<dyn StaticCommand>;
        let mut started = false;

        for i in (commands.len()-2)..0 {
            let (head, tail) = (commands[i].borrow(), commands[i+1].borrow());
            curr = match head {
                
            };
        }

        StaticSolution { 
            head: curr,
        }
    }

    fn run(&self, input: f64) -> f64 {
        self.head.operate(input)
    }
}