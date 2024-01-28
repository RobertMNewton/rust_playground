use super::benchmark::{Solution, Commands};

trait DynamicCommand {
    fn operate(&self, input: f64) -> f64;
    fn set_next(&mut self, next: Box<dyn DynamicCommand>);
}

macro_rules! impl_dynamic_command {
    ($name:ident, $op:expr) => {
        pub struct $name {
            next: Option<Box<dyn DynamicCommand>>,
        }

        impl $name {
            fn new(next: Option<Box<dyn DynamicCommand>>) -> Self {
                Self { next }
            }
        }

        impl DynamicCommand for $name {
            fn operate(&self, x: f64) -> f64 {
                match &self.next {
                    Some(command) => command.operate($op(x)),
                    None => $op(x),
                }
            }

            fn set_next(&mut self, next: Box<dyn DynamicCommand>) {
                self.next = Some(next);
            }
        }
    };
}

impl_dynamic_command!(Half, |x| x / 2.0);
impl_dynamic_command!(Double, |x| x * 2.0);
impl_dynamic_command!(Increment, |x| x + 1.0);
impl_dynamic_command!(Decrement, |x| x - 1.0);
impl_dynamic_command!(Square, |x: f64| x.powf(2.0));
impl_dynamic_command!(Root, |x: f64| x.powf(0.5));

pub struct DynamicSolution {
    head: Box<dyn DynamicCommand>,
}

impl Solution for DynamicSolution {
    fn new(commands: Vec<Commands>) -> Self {
        let mut dynamic_commands: Vec<Box<dyn DynamicCommand>> = commands.into_iter()
            .map(|command| {
                match command {
                    Commands::Half => Box::new(Half::new(None)) as Box<dyn DynamicCommand>,
                    Commands::Double => Box::new(Double::new(None)) as Box<dyn DynamicCommand>,
                    Commands::Increment => Box::new(Increment::new(None)) as Box<dyn DynamicCommand>,
                    Commands::Decrement => Box::new(Decrement::new(None)) as Box<dyn DynamicCommand>,
                    Commands::Square => Box::new(Square::new(None)) as Box<dyn DynamicCommand>,
                    Commands::Root => Box::new(Root::new(None)) as Box<dyn DynamicCommand>,
                }
            })
            .collect();

        let mut head: Box<dyn DynamicCommand> = dynamic_commands.pop().unwrap();
        while !dynamic_commands.is_empty() {
            let mut current = dynamic_commands.pop().unwrap();
            current.set_next(head.into());
            head = current;
        }


    DynamicSolution { 
        head: head,
    }
    }

    fn run(&self, input: f64) -> f64 {
        self.head.operate(input)
    }
}