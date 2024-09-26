use ode_solvers::{dop_shared::OutputType, Dopri5, System, Vector1};

type State = Vector1<f64>; // type State = OVector<f32, U3>;
type Time = f64;
// type Result = SolverResult<Time, State>;

struct MyThingToSolve {}

impl System<Time, State> for MyThingToSolve {
    fn system(&self, _x: Time, y: &State, dy: &mut State) {
        dy[0] = -0.5 * y[0]
    }
}

struct MyThingToSolveWithEarlyExitCondition {}

impl System<Time, State> for MyThingToSolveWithEarlyExitCondition {
    fn system(&self, _x: Time, y: &State, dy: &mut State) {
        dy[0] = -0.5 * y[0]
    }
    
    // Stop function called at every successful integration step. The integration is stopped when this function returns true.
    fn solout(&mut self, x: Time, y: &State, dy: &State) -> bool {
        dbg!(x, y, dy);
        y[0] < 0.3
    }
}

fn main() {
    let f = MyThingToSolveWithEarlyExitCondition {}; // f - Structure implementing the System trait
    let x: Time = 0.; // x - Initial value of the independent variable (usually time)
    let x_end: Time = 5.; // x_end - Final value of the independent variable

    // Problem 1 - Python doesn't take a timestep (dx). Seems to figure it out itself...
    let dx = 0.; // dx - Increment in the dense output. This argument has no effect if the output type is Sparse

    let y0: State = State::new(2.); // y - Initial value of the dependent variable(s)

    // scipy docs: Default values are 1e-3 for rtol and 1e-6 for atol.

    // scipy implementation for reference:
    // https://github.com/scipy/scipy/blob/6b657ede0c3c4cffef3156229afddf02a2b1d99a/scipy/integrate/_ivp/rk.py#L293
    let rtol = 1e-3; // rtol - set from scipy docs - Relative tolerance used in the computation of the adaptive step size
    let atol = 1e-6; // atol - set from scipy docs - Absolute tolerance used in the computation of the adaptive step size
    let h = 0.; // initial step size - 0
    let safety_factor = 0.9; // matches scipy implementation
    let beta = 0.; // setting this to 0 gives us an alpha of 0.2 and matches scipy's adaptive step size logic (default was 0.04)
    let fac_min = 0.2; // matches scipy implementation
    let fac_max = 10.; // matches scipy implementation
    let h_max = x_end-x;
    let n_max = 100000;
    let n_stiff = 1000;

    let mut stepper = Dopri5::from_param(f, x, x_end, dx, y0, rtol, atol, safety_factor, beta, fac_min, fac_max, h_max, h, n_max, n_stiff, OutputType::Sparse);

    let integration_statistics = stepper.integrate();
    println!("Integration stats: {:?}", integration_statistics);

    let x_output = stepper.x_out(); // Getter for the independent variable's output
    let y_output = stepper.y_out(); // Getter for the dependent variables' output
    println!("X Output: {:?}", x_output);
    println!("Y Output: {:?}", y_output);
    println!("Last Y Output: {:?}", y_output.last());

    // commented out for now - results looks to be a combination of x_output and y_output
    // let result = stepper.results();
    // println!("Result: {:?}", result);
}