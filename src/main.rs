extern crate rand;
use rand::{thread_rng, Rng};

fn interval(q:f32, a: f32, b: f32) -> bool {
    if (q > a) && (q <= b) {
        return true
    }
    false
}

struct State {
    name: String,
    links: Vec<(usize, f32)>
}

impl State {
    fn new(name: String) -> State {
        State{links: vec![], name: name}
    }

    fn mklink(&mut self, a: usize, pa: f32) {
        for link in self.links.iter_mut() {
            if link.0 == a {
                link.1 = link.1 + ((1 as f32 - link.1) * pa);
                return
            }
        }
        self.links.push((a, pa));      
    }

    fn next(&mut self) -> usize {
        let mut rng = thread_rng();
        let mut val = rng.gen_range(0.0, 1.0);
        self.links.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut floor = 0.0;
        for link in self.links.iter() {
            if interval(val, floor, link.1) {
                //println!("\n seed: {} matched to ({}, {})", val, floor, link.1);
                return link.0
            } else {
                floor = link.1;
            }
        }
        0
    }
}

struct MarkovChain {
    current: usize,
    states: Vec<State>
}

impl MarkovChain {
    fn new() -> MarkovChain {
        MarkovChain{states: vec![], current: 0 as usize}
    }

    fn add_state(&mut self, s: State) {
        self.states.push(s);
    }

    fn add_transition(&mut self, a: usize, b: usize, pb: f32) {
        self.states[a].mklink(b, pb);
    }

    fn simulate(&mut self, transitions: usize) {
        println!("Simulating {} transitions...", transitions);
        println!("Beginning with state: {}", self.states[self.current].name);
        for i in 0..transitions {
            print!("\tt{:<3}: {} ->", i + 1, self.states[self.current].name);
            self.step();
            println!(" {}", self.states[self.current].name);
        }
    }

    fn step(&mut self) {
        self.current = self.states[self.current].next();
    }

    fn repr(&self) {
        println!("");
        for state in self.states.iter() {
            println!("{} links to:", state.name);
            for link in state.links.iter() {
                println!("\t{}: {}", self.states[link.0].name, link.1);
            }
            println!("");
        }
    }
}

fn main() {
    assert!(interval(0.5,  0.0, 1.0));
    let mut chain = MarkovChain::new();
    chain.add_state(State::new(String::from("Sunny")));
    chain.add_state(State::new(String::from("Rainy")));
    chain.add_transition(0, 0, 0.1);
    chain.add_transition(0, 1, 0.9);
    chain.add_transition(1, 0, 0.2);
    chain.add_transition(1, 1, 0.8);
    chain.repr();
    chain.simulate(10);
}
