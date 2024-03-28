use std::iter::once;

#[derive(Clone)]
pub struct Wheel {
    pub basis: Vec<usize>,
    pub spokes: Vec<usize>,
}

impl Wheel {
    pub fn circumference(&self) -> usize {
        self.basis.iter().product()
    }
}

fn next_wheel(wheel: Wheel) -> Wheel {
    let p = if wheel.spokes.len() > 1 { wheel.spokes[1] } else { 3 };
    let circ = wheel.circumference();
    let basis = wheel.basis.iter().copied().chain(once(p)).collect::<Vec<usize>>();
    let mut spokes: Vec<usize> = vec![];
    for k in 0..p {
        for s in &wheel.spokes {
            let spoke = k * circ + s;
            if spoke % p != 0 {
                spokes.push(spoke);
            }
        }
    }
    Wheel { basis, spokes }
}

pub fn get_wheel(basis_size: usize) -> Wheel {
    let mut wheel = Wheel { basis: vec![2], spokes: vec![1] };
    for _ in 1..basis_size {
        wheel = next_wheel(wheel);
    }
    wheel
}