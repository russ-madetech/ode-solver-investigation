use nalgebra::{vector, OVector, U3};
use ode_solvers::{dop_shared::{FloatNumber, OutputType}, Dopri5, Rk4, System, Vector1, Vector3};

type State = Vector1<f64>; // type State = OVector<f32, U3>;
type Time = f64;
// type Result = SolverResult<Time, State>;

struct MyThingToSolve {}

impl System<Time, State> for MyThingToSolve {
    fn system(&self, _x: Time, y: &State, dy: &mut State) {
        dy[0] = -0.5 * y[0]
    }

    // TODO there's a solout function we can implement here
    // may give us the behaviour we need to exit early
}

fn main() {
    let f = MyThingToSolve {}; // f - Structure implementing the System trait
    let x: Time = 0.; // x - Initial value of the independent variable (usually time)
    let x_end: Time = 5.; // x_end - Final value of the independent variable

    // Problem 1 - Python doesn't take a timestep (dx). Seems to figure it out itself...
    let dx = 0.1; // dx - Increment in the dense output. This argument has no effect if the output type is Sparse
     
    let y0: State = State::new(2.); // y - Initial value of the dependent variable(s)

    // scipy docs: Default values are 1e-3 for rtol and 1e-6 for atol.

    let rtol = 1e-3; // rtol - Relative tolerance used in the computation of the adaptive step size
    let atol = 1e-6; // 0.0000000001; // atol - Absolute tolerance used in the computation of the adaptive step size
    let h = 0.;
    let safety_factor = 1.;
    let beta = 0.04; // based on https://github.com/scipy/scipy/blob/6b657ede0c3c4cffef3156229afddf02a2b1d99a/scipy/integrate/_ivp/rk.py#L293   // default was: 0.04; // ?
    let fac_min = 0.2;
    let fac_max = 100.;
    let h_max = x_end-x;
    let n_max = 100000;
    let n_stiff = 1000;

    // ode_solvers has a adaptive step function which gives similar (but different) results to the one used 
    // by scipy solve_ivp.

    // next steps:
    // - can we replicate the adaptive step size function?
    // - is it good enough without doing that?
    let mut stepper = Dopri5::from_param(f, x, x_end, dx, y0, rtol, atol, safety_factor, beta, fac_min, fac_max, h_max, h, n_max, n_stiff, OutputType::Sparse);

    let integration_statistics = stepper.integrate();
    println!("Integration stats: {:?}", integration_statistics);

    let x_output = stepper.x_out(); // Getter for the independent variable's output
    let y_output = stepper.y_out(); // Getter for the dependent variables' output
    println!("X Output: {:?}", x_output);
    println!("Y Output: {:?}", y_output);

    let result = stepper.results();

    println!("Result: {:?}", result);
}