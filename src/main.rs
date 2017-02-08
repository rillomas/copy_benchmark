extern crate rand;
extern crate time;

fn main() {
    let size = 4*1024*1024;
    let mut in_buf = Vec::<f32>::new();
    let mut out_buf = Vec::<f32>::new();
    in_buf.resize(size, 0.0f32);
    out_buf.resize(size, 0.0f32);
    let iter_num = 10;
    for _ in 0..iter_num {
        for i in 0..size {
            in_buf[i] = rand::random::<f32>();
        }
        let start = time::precise_time_ns();
        out_buf.copy_from_slice(&in_buf);
        let end = time::precise_time_ns();
        let dur = end - start;
        let msec = (dur as f32) * 0.001 * 0.001; // ns to ms
        println!("time: {:.2} ms", msec);
        for i in 0..size {
            let exp = in_buf[i];
            let act = out_buf[i];
            if exp != act {
                println!("Results mismatch at {}: expected {} actual {}", i, exp, act);
                std::process::exit(-1) 
            }
        }
    }
    std::process::exit(0) 
}