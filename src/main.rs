use parallel::*;

fn main() {
    println!("Parallel: start");
    show_cpus();
    monte_carlo::pi::main();
    println!("Parallel: done");
}

fn show_cpus() {
    println!("cores = {}", num_cpus::get_physical());
    println!("cpus = {}", num_cpus::get());
}