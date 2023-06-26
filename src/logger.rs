use atty::Stream;
use kdam::{term::Colorizer, tqdm, BarExt, Column, RichProgress};
use std::{sync::{Arc, Mutex}, fmt};

pub struct Logger {
    pub total_bar: Option<RichProgress>,
}

impl Logger {
    pub fn init() -> Self {
        if atty::isnt(Stream::Stdout) {
            println!(
                "mrxbuilder version {}",
                String::from(env!("CARGO_PKG_VERSION"))
            );
            println!("github.com/mutant-remix/mrxbuilder");
            println!("Running in tty mode with pretty printing disabled!");
            println!();

            return Self {
                total_bar: None,
            };
        }

        #[rustfmt::skip]
        println!(
            r"
    {}
    {}   {}
    {}   {}: {}
    {}   {}
    {}
            ",
            "⠀⠀⣀⠤⠤⠤⠤⣀⠀⠀".colorize("bright bold white"),
            "⢀⠞⠀⡀⠀⠀⢀⠀⢣⡀".colorize("bright bold white"), "mrxbuilder".colorize("bold magenta"),
            "⣾⠀⢰⡿⠦⠴⢿⡆⠀⣷".colorize("bright bold white"), "Version".colorize("bold blue"), String::from(env!("CARGO_PKG_VERSION")).colorize("green"),
            "⣧⠀⠈⠗⠀⠀⠺⠁⠀⣼".colorize("bright bold white"), "github.com/mutant-remix/mrxbuilder".colorize("dimmed white"),
            "⠈⠑⠒⠒⠒⠒⠒⠒⠊⠁".colorize("bright bold white")
        );

        let mut total_bar = RichProgress::new(
            tqdm!(total = 1, force_refresh = true, position = 0),
            vec![
                Column::text("[bold cyan]Total"),
                Column::Bar,
                Column::CountTotal,
                Column::text("Elapsed:"),
                Column::ElapsedTime,
            ],
        );

        total_bar.update(1);

        Logger {
            total_bar: Some(total_bar),
        }
    }

    pub fn set_stage(&mut self, message: &str, size: usize) -> Arc<Mutex<RichProgress>> {
        if self.total_bar.is_none() {
            println!("INFO  {}", message);
            return Arc::new(Mutex::new(RichProgress {
                pb: tqdm!(),
                columns: vec![],
            }));
        }

        self.total_bar.as_mut().unwrap().update(1);

        let current_bar = RichProgress::new(
            tqdm!(total = size, force_refresh = false, position = 1),
            vec![
                Column::text(&format!("[bold green]{}", message)),
                Column::Bar,
                Column::Percentage(1),
                Column::CountTotal,
                Column::Rate,
                Column::text("ETA:"),
                Column::RemainingTime,
            ],
        );

        Arc::new(Mutex::new(current_bar))
    }

    pub fn set_stage_count(&mut self, size: usize) {
        match self.total_bar {
            Some(ref mut bar) => {
                bar.pb.set_total(size);
            }
            None => {}
        }
    }

    pub fn load(&mut self, message: &str) {
        match self.total_bar {
            Some(ref mut bar) => {
                bar.write(format!("{} {}", "LOAD ".colorize("bold yellow"), message));
            }
            None => {
                println!("LOAD {}", message);
            }
        }
    }

    pub fn info(&mut self, message: &str) {
        match self.total_bar {
            Some(ref mut bar) => {
                bar.write(format!("{} {}", "INFO ".colorize("bold blue"), message));
            }
            None => {
                println!("INFO  {}", message);
            }
        }
    }

    pub fn build(&mut self, message: &str) {
        match self.total_bar {
            Some(ref mut bar) => {
                bar.write(format!("{} {}", "BUILD".colorize("bold magenta"), message));
            }
            None => {
                println!("BUILD {}", message);
            }
        }
    }

    pub fn error(&mut self, message: &str) {
        match self.total_bar {
            Some(ref mut bar) => {
                bar.write(format!("{} {}", "ERROR".colorize("bold red"), message));
            }
            None => {
                println!("ERROR {}", message);
            }
        }

        std::process::exit(1);
    }

    pub fn finish(&mut self) {
        match self.total_bar {
            Some(ref mut bar) => {
                let elapsed = bar.pb.elapsed_time();
                let elapsed = (elapsed * 100.0).round() / 100.0;

                bar.write(format!(
                    "{} in {}s",
                    "DONE ".colorize("bold green"),
                    elapsed
                ));

                bar.clear();
            }
            None => {
                println!("DONE");
            }
        }
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &format!("<Logger>")
        )
    }
}
