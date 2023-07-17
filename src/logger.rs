use atty::Stream;
use kdam::{term::Colorizer, tqdm, BarExt, Column, RichProgress};
use std::{
    fmt, panic,
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct Logger {
    total_bar: Bar,
}

#[derive(Clone)]
pub enum Bar {
    Notty(Instant),
    Tty(Arc<Mutex<RichProgress>>),
}

impl Bar {
    pub fn new(bar: Option<RichProgress>) -> Self {
        match bar {
            Some(bar) => Self::Tty(Arc::new(Mutex::new(bar))),
            None => Self::Notty(Instant::now()),
        }
    }

    pub fn inc(&mut self) {
        match self {
            Self::Notty(_) => {}
            Self::Tty(bar) => {
                bar.lock().unwrap().update(1);
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Notty(_) => {}
            Self::Tty(bar) => {
                bar.lock().unwrap().clear();
            }
        }
    }

    pub fn write(&mut self, message: &str) {
        match self {
            Self::Notty(_) => {}
            Self::Tty(bar) => {
                bar.lock().unwrap().write(message);
            }
        }
    }

    pub fn set_total(&mut self, total: usize) {
        match self {
            Self::Notty(_) => {}
            Self::Tty(bar) => {
                bar.lock().unwrap().pb.set_total(total);
            }
        }
    }

    pub fn elapsed_time(&self) -> f32 {
        match self {
            Self::Notty(start) => start.elapsed().as_secs_f32(),
            Self::Tty(bar) => bar.lock().unwrap().pb.elapsed_time(),
        }
    }
}

impl Logger {
    pub fn init() -> Self {
        if atty::isnt(Stream::Stdout) {
            println!(
                "mrxbuilder version {}",
                String::from(env!("CARGO_PKG_VERSION"))
            );
            println!("github.com/mutant-remix/mrxbuilder");
            println!(
                "Running in no tty mode with pretty printing disabled due to unsupported terminal"
            );
            println!();

            return Self {
                total_bar: Bar::new(None),
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

        let total_bar = RichProgress::new(
            tqdm!(total = 1, force_refresh = true, position = 0),
            vec![
                Column::text("[bold cyan]Total"),
                Column::Bar,
                Column::CountTotal,
                Column::text("Elapsed:"),
                Column::ElapsedTime,
            ],
        );

        Logger {
            total_bar: Bar::new(Some(total_bar)),
        }
    }

    pub fn new_stage(&mut self, message: &str, size: usize) -> Bar {
        self.total_bar.inc();

        match &self.total_bar {
            Bar::Tty(_) => {
                let current_bar = RichProgress::new(
                    tqdm!(total = size, force_refresh = true, position = 1),
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

                Bar::new(Some(current_bar))
            }
            Bar::Notty(_) => Bar::new(None),
        }
    }

    pub fn register_panic_hook(&mut self) {
        let bar = Mutex::new(self.total_bar.clone());

        panic::set_hook(Box::new(move |panic_info| {
            let error = panic_info.to_string();
            let mut bar = bar.lock().unwrap();

            match bar.to_owned() {
                Bar::Tty(_) => {
                    bar.write(&format!("{} {}", "FATAL".colorize("bold red"), error));
                    bar.clear();
                }
                Bar::Notty(_) => {
                    println!("FATAL {}", error);
                }
            }
        }));
    }

    pub fn set_stage_count(&mut self, size: usize) {
        self.total_bar.set_total(size);
    }

    pub fn load(&mut self, message: &str) {
        match &mut self.total_bar {
            Bar::Tty(_) => {
                self.total_bar
                    .write(&format!("{} {}", "LOAD ".colorize("bold yellow"), message));
            }
            Bar::Notty(_) => {
                println!("LOAD {}", message);
            }
        }
    }

    pub fn info(&mut self, message: &str) {
        match &mut self.total_bar {
            Bar::Tty(_) => {
                self.total_bar
                    .write(&format!("{} {}", "INFO ".colorize("bold blue"), message));
            }
            Bar::Notty(_) => {
                println!("INFO  {}", message);
            }
        }
    }

    pub fn build(&mut self, message: &str) {
        match &mut self.total_bar {
            Bar::Tty(_) => {
                self.total_bar
                    .write(&format!("{} {}", "BUILD".colorize("bold magenta"), message));
            }
            Bar::Notty(_) => {
                println!("BUILD {}", message);
            }
        }
    }

    pub fn finish(&mut self) {
        let elapsed = self.total_bar.elapsed_time();
        let elapsed = (elapsed * 100.0).round() / 100.0;

        match &mut self.total_bar {
            Bar::Tty(_) => {
                self.total_bar.write(&format!(
                    "{} in {}s",
                    "DONE ".colorize("bold green"),
                    elapsed
                ));

                self.total_bar.clear();
            }
            Bar::Notty(_) => {
                println!("DONE in {}s", elapsed);
            }
        }
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("<Logger>"))
    }
}
