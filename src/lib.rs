struct OrdinaryDifferentialEquation {
    name: String,
    ode: fn(f64) -> f64,
    initial_pt: f64,
    initial_value: f64,
}

impl OrdinaryDifferentialEquation {
    fn new(name: &str, ode: fn(f64) -> f64, initial_pt: f64, initial_value: f64) -> Self {
        OrdinaryDifferentialEquation {
            name: (*name).to_string(),
            ode,
            initial_pt,
            initial_value,
        }
    }

    fn evaluate_at(&self, at: f64) -> f64 {
        (self.ode)(at)
    }
}

struct OdeSolver<'a> {
    ode: &'a OrdinaryDifferentialEquation,
    step_size: f64,
}

impl<'a> OdeSolver<'a> {
    fn new(ode: &'a OrdinaryDifferentialEquation, step_size: f64) -> Self {
        OdeSolver { ode, step_size }
    }

    fn solve_at(&self, at: f64) -> f64 {
        let mut current_step = self.ode.initial_pt;
        let mut current_value = self.ode.initial_value;
        while current_step < at {
            current_step += self.step_size;
            current_value += self.step_size * self.ode.evaluate_at(current_value);
        }
        current_value
    }
}

#[cfg(test)]
mod tests {
    use crate::OdeSolver;
    use crate::OrdinaryDifferentialEquation;

    fn simple_ode(input: f64) -> f64 {
        input
    }

    #[test]
    fn test_simple_ode() {
        let ode = OrdinaryDifferentialEquation::new("simple_ode", simple_ode, 0f64, 1f64);
        assert_eq!(50.0, ode.evaluate_at(50.0));
    }

    #[test]
    fn test_solve_simple_ode() {
        let ode = OrdinaryDifferentialEquation::new("simple_ode", simple_ode, 0f64, 1f64);
        let solver = OdeSolver::new(&ode, 1.0);
        assert_eq!(16.0, solver.solve_at(4.0));
    }
}
