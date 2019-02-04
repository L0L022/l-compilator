extern crate exitfailure;

extern crate l_compilator;

fn main() -> Result<(), exitfailure::ExitFailure> {
    Ok(l_compilator::App::run()?)
}
