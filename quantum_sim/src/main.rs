use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Debug, Clone)]
struct Qubit {
    alpha: Complex64,
    beta: Complex64,
}

impl Qubit {
    fn new() -> Self {
        Self {
            alpha: Complex64::new(1.0, 0.0),
            beta: Complex64::new(0.0, 0.0),
        }
    }

    fn x(&mut self) {
        std::mem::swap(&mut self.alpha, &mut self.beta);
    }

    fn h(&mut self) {
        let a = self.alpha;
        let b = self.beta;
        let inv_sqrt2 = FRAC_1_SQRT_2;
        
        self.alpha = (a + b) * inv_sqrt2;
        self.beta = (a - b) * inv_sqrt2;
    }

    fn measure(&mut self) -> usize {
        let prob_0 = self.alpha.norm_sqr();
        let random_shot = rand::random::<f64>(); 
        
        if random_shot < prob_0 {
            self.alpha = Complex64::new(1.0, 0.0);
            self.beta = Complex64::new(0.0, 0.0);
            0
        } else {
            self.alpha = Complex64::new(0.0, 0.0);
            self.beta = Complex64::new(1.0, 0.0);
            1
        }
    }
}

fn main() {
    println!("--- Initializing Qubit in State |0> ---");
    let mut qubit = Qubit::new();
    println!("{:?}\n", qubit);

    println!("--- Applying Hadamard Gate (Superposition) ---");
    qubit.h();
    println!("{:?}", qubit);
    println!("Probability of |0>: {:.2}%", qubit.alpha.norm_sqr() * 100.0);
    println!("Probability of |1>: {:.2}%\n", qubit.beta.norm_sqr() * 100.0);

    println!("--- Measuring the Qubit 5 times (Re-initializing each time) ---");
    for i in 1..=5 {
        let mut sim_qubit = qubit.clone();
        let result = sim_qubit.measure();
        println!("Shot {}: Measured |{}>", i, result);
    }
}
