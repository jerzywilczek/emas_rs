use emas_rs::SystemBuilder;

fn main() {
    let mut system = SystemBuilder::new().build();
    let sol = system.run();
    println!("[{}, {}]", sol[0], sol[1]);
}