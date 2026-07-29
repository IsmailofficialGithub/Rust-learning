use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

fn main() {
    // --- knobs (raise carefully) ---
    const CPU_PERCENT: usize = 1120; // 120% of logical cores
    const MEMORY_MB: usize = 12048; // committed RAM for this process
    const DURATION_SECS: u64 = 1130;

    let logical_cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let cpu_threads = (logical_cores * CPU_PERCENT).div_ceil(100).max(1);

    println!("=== Machine capacity stress test ===");
    println!("Logical cores : {logical_cores}");
    println!("Worker threads: {cpu_threads} ({CPU_PERCENT}% of cores)");
    println!("Memory target : {MEMORY_MB} MB");
    println!("Duration      : {DURATION_SECS}s");
    println!("Save work first — the machine will get slow.\n");

    println!("Allocating {MEMORY_MB} MB...");
    let mut memory = vec![0u8; MEMORY_MB * 1024 * 1024];

    // Touch every page so RAM is actually committed (not just virtual).
    for i in (0..memory.len()).step_by(4096) {
        memory[i] = (i % 255) as u8;
    }
    println!("Memory committed.\n");

    let running = Arc::new(AtomicBool::new(true));
    let mut handles = Vec::with_capacity(cpu_threads);

    for id in 0..cpu_threads {
        let running = running.clone();
        handles.push(thread::spawn(move || {
            let mut x: u64 = id as u64 + 1;
            while running.load(Ordering::Relaxed) {
                for _ in 0..5_000_000 {
                    x = x
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1);
                    std::hint::black_box(x);
                }
            }
            x
        }));
    }

    let start = Instant::now();
    println!("Burning CPU for {DURATION_SECS} seconds...");

    while start.elapsed() < Duration::from_secs(DURATION_SECS) {
        thread::sleep(Duration::from_secs(1));
        println!("Elapsed: {}s", start.elapsed().as_secs());
    }

    running.store(false, Ordering::Relaxed);
    for handle in handles {
        let _ = handle.join();
    }

    // Keep memory live until the end so Task Manager shows the peak.
    std::hint::black_box(&memory);

    println!("Finished cleanly.");
}
