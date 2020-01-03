use std::cell::Cell;

use cucumber::cucumber;

#[derive(std::default::Default)]
pub struct Context {}

impl cucumber::World for Context {}

mod example_steps {
    use cucumber::steps;

    steps!(crate::Context => {
        given "a scenario" |_world, _step| {
        };
    });
}

thread_local!(static SETUP_INVOCATIONS: Cell<usize> = Cell::new(0));

// A setup function to be called before everything else
fn setup() {
    SETUP_INVOCATIONS.with(|invocations| {
        let count = invocations.get() + 1;
        invocations.set(count);
        println!("Setup has run {} times", count);
    });
}

cucumber! {
    features: "./features",
    world: crate::Context,
    steps: &[example_steps::steps],
    setup: setup
}
