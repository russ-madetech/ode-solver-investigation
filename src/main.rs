use nalgebra::{vector, OVector, U3};
use ode_solvers::{Dopri5, Rk4, System, Vector3};

type State = Vector3<f64>; // type State = OVector<f32, U3>;
type Time = f64;
// type Result = SolverResult<Time, State>;

struct MyThingToSolve {}

impl System<Time, State> for MyThingToSolve {
    fn system(&self, _x: Time, y: &State, dy: &mut State) {
        dy[0] = -0.5 * y[0]
    }
}

fn main() {
    let f = MyThingToSolve {}; // f - Structure implementing the System trait
    let x: Time = 0.; // x - Initial value of the independent variable (usually time)
    let x_end: Time = 5.; // x_end - Final value of the independent variable
    let dx = 0.01; // dx - Increment in the dense output. This argument has no effect if the output type is Sparse
                   // let y0: State = vector![2., 5., 8.]; // y - Initial value of the dependent variable(s)
    let y0: State = State::new(2., 5., 8.); // y - Initial value of the dependent variable(s)
    let rtol = 0.0000000001; // rtol - Relative tolerance used in the computation of the adaptive step size
    let atol = 0.0000000001; // atol - Absolute tolerance used in the computation of the adaptive step size
                             // let step_size: f32 = 0.01;

    let mut stepper = Dopri5::new(f, x, x_end, dx, y0, rtol, atol);
    let x_output = stepper.x_out(); // Getter for the independent variable's output
    let y_output = stepper.y_out(); // Getter for the dependent variables' output
    println!("X Output: {:?}", x_output);
    println!("Y Output: {:?}", y_output);
    let result = stepper.results();
    println!("Result: {:?}", result);

    let integration_statistics = stepper.integrate();
    println!("Integration stats: {:?}", integration_statistics);
}