#[allow(unused_macros)]

cfg_if! {
    if #[cfg(feature = "wasm")] {
        macro_rules! log {
            ($($t:tt)*) => ()
        }

        macro_rules! elog {
            ($($t:tt)*) => ()
        }

        macro_rules! log_verbose {
            ($($t:tt)*) => ()
        }

        macro_rules! elog_verbose {
            ($($t:tt)*) => ()
        }

        pub struct Stopwatch {}

        impl Stopwatch {
            pub fn new() -> Stopwatch {
                Stopwatch {}
            }
        }
    } else {
        macro_rules! log {
            ($($t:tt)*) => (println!($($t)*))
        }

        macro_rules! elog {
            ($($t:tt)*) => (eprintln!($($t)*))
        }

        macro_rules! log_verbose {
            ($($t:tt)*) => (if $crate::verbose_flag() { println!($($t)*) })
        }

        macro_rules! elog_verbose {
            ($($t:tt)*) => (if $crate::verbose_flag() { eprintln!($($t)*) })
        }

        pub struct Stopwatch {
            start: std::time::Instant
        }

        impl Stopwatch {
            pub fn new() -> Stopwatch {
                Stopwatch { start: std::time::Instant::now() }
            }

            pub fn elapsed(&self) -> f64 {
                self.start.elapsed().as_millis() as f64 / 1000.0
            }
        }
    }
}