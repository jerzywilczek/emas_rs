let start_time = Instant::now();
let solution = system.run();
let end_time = start_time.elapsed().as_secs_f32();
println!("time = {}, sulution = [{}, {}] => {}",
         end_time, solution[0], solution[1], rosenbrock_2d(&solution));