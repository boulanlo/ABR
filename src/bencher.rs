use crate::abr::ABR;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use std::time::Duration;
use time::precise_time_ns;

const ITERATIONS: usize = 100;

pub struct Bencher<'a, S, F> {
    path: &'a Path,
    setup: S,
    function: F,
}

impl<'a, S, F> Bencher<'a, S, F>
where
    S: Fn(usize) -> ABR<u64, ()>,
    F: Fn(ABR<u64, ()>, Option<usize>) -> ABR<u64, ()>,
{
    pub fn new(path: &'a Path, setup: S, function: F) -> Bencher<'a, S, F> {
        std::fs::create_dir_all(path).expect("Could not create benchmark directory");
        Bencher {
            path,
            setup,
            function,
        }
    }

    fn bench(&self, size: usize, levels: Option<usize>) -> Duration {
        let mut results: Vec<u64> = vec![];

        for i in 0..ITERATIONS {
            let setup_result = (self.setup)(size);

            print!(
                "\r{: <40}\rIteration {}/{} ({}%)...",
                "",
                i + 1,
                ITERATIONS,
                ((i as f32 / ITERATIONS as f32) * 100.0)
            );

            std::io::stdout()
                .flush()
                .expect("error during debug printing");

            let begin = precise_time_ns();

            let _ = (self.function)(setup_result, levels); // the let is to avoid measuring tree drop in measurements

            let end = precise_time_ns();

            results.push(end - begin);
        }

        println!();

        Duration::from_nanos(results.iter().cloned().sum::<u64>() / (ITERATIONS as u64))
    }

    fn log(
        &self,
        data_writer: &mut BufWriter<File>,
        const_writer: &mut BufWriter<File>,
        result: Duration,
        level: Option<usize>,
    ) -> Result<(), Error> {
        if let Some(l) = level {
            data_writer.write_all(format!("{} {}\n", l, result.as_nanos()).as_bytes())?;
        } else {
            const_writer.write_all(format!("{}\n", result.as_nanos()).as_bytes())?;
        }
        Ok(())
    }

    pub fn run_benchmark(
        &self,
        name: &str,
        sizes: Vec<usize>,
        levels: Vec<Option<usize>>,
    ) -> Result<(), Error> {
        println!("Running {}...", name);

        for size in &sizes {
            let data_file = File::create(self.path.join(format!("{}.data", size)))?;
            let const_file = File::create(self.path.join(format!("{}.constant", size)))?;

            let mut data_writer = BufWriter::new(data_file);
            let mut const_writer = BufWriter::new(const_file);

            for level in &levels {
                let result = self.bench(*size, *level);
                println!(
                    "Size {}, level {:?} : {} µs.",
                    size,
                    level,
                    result.as_micros()
                );
                self.log(&mut data_writer, &mut const_writer, result, *level)?;
            }
        }

        Ok(())
    }
}
