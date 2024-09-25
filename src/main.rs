use ode_solvers::{Dopri5, Rk4, System, Vector1, Vector3};
use nalgebra::{vector, Const, OVector, U1};

type State = OVector<f32, U1>;
type Time = f32;
// type Result = SolverResult<Time, State>;

struct MyThingToSolve {
}

impl System<Time,State> for MyThingToSolve {
    fn system(&self, x: Time, y: &State, dy: &mut State) {
        dy[0] = 0.5 * y[0]
    }
} 

fn main() {
    let x: Time = 0.;
    let y0: State = vec![2., 5., 8.]; // vector![2., 5., 8.];
    let dx = todo!();
    let x_end: Time = 5.;
    let step_size: f32 = 0.01; 
    let f = MyThingToSolve {};
    let rtol = todo!();
    let atol = todo!();
    let mut stepper = Dopri5::new(f, x, x_end, dx, y0, rtol, atol);
}