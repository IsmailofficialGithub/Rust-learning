use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use rand::RngExt;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let racer1 = F1Racer::new("Max Verstappen");
    let racer2 = F1Racer::new("Lewis Hamilton");

    println!("========== Race Started ==========\n");

    let t1 = tokio::spawn(race(racer1));
    let t2 = tokio::spawn(race(racer2));

    let (best1, best2) = tokio::join!(t1, t2);

    println!("\n========== Race Finished ==========");
    println!("Max Best Lap   : {:.1}", best1.unwrap());
    println!("Lewis Best Lap : {:.1}", best2.unwrap());
}

struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: f32,
    lap_times: Vec<f32>,

    // Timer for the current lap
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
}

impl F1Racer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            completed_laps: 0,
            laps: 5,
            best_lap_time: f32::MAX,
            lap_times: vec![8.0, 7.5, 8.2, 7.9, 8.1],
            sleep: None,
        }
    }
}

impl Future for F1Racer {
    type Output = f32;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let racer = self.get_mut();

        // Finished?
        if racer.completed_laps == racer.laps {
            println!("{} finished the race!", racer.name);
            return Poll::Ready(racer.best_lap_time);
        }

        // Create a timer for this lap if we don't have one yet.
        // Important: do NOT return Pending here — fall through and poll the
        // Sleep so it can register the waker with Tokio's timer.
        if racer.sleep.is_none() {
            let mut rng = rand::rng();
            let delay = rng.random_range(1..=3);

            println!(
                "{} starts lap {} ({} sec)",
                racer.name,
                racer.completed_laps + 1,
                delay
            );

            racer.sleep = Some(Box::pin(tokio::time::sleep(
                Duration::from_secs(delay),
            )));
        }

        // Poll the timer (this is what registers the waker)
        let timer = racer.sleep.as_mut().unwrap();

        match timer.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,

            Poll::Ready(()) => {
                racer.sleep = None;

                let lap_time = racer.lap_times.remove(0);

                racer.completed_laps += 1;

                if lap_time < racer.best_lap_time {
                    racer.best_lap_time = lap_time;
                }

                println!(
                    "{} completed lap {} in {:.1} sec (Best {:.1})",
                    racer.name,
                    racer.completed_laps,
                    lap_time,
                    racer.best_lap_time
                );

                // Ask Tokio to poll us again immediately so the next lap's timer
                // can be created.
                cx.waker().wake_by_ref();

                Poll::Pending
            }
        }
    }
}

async fn race(racer: F1Racer) -> f32 {
    racer.await
}