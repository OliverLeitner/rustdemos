/**
 * our logging facility
 */
use log::{debug, error, info, warn, log_enabled, Level, LevelFilter};
// use std::io::Write; // write stuff
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub struct LoggingService {
    pub filename_prefix: String,
    pub filename_suffix: String,
    pub file_log_path: String
}

// TODO: lets log into files, which are datestamped, in env logs defined dir
impl LoggingService {
    pub fn logger(self: &Self) -> Result<(), String> {
        // std::env::set_var("RUST_LOG", "actix_web=debug");
        // env_logger::init();
        self.createLogFile();
        Result::Ok(()) // void in rust
    }

    pub fn createLogFile(self: &Self) {
        let cTime = chrono::Utc::now();
        let timeStamp = cTime.format("%Y%m%d%H%M%S%f").to_string();
        let logFileName = format!(
            "{}/{}-{}-{}.log",
            self.file_log_path,
            self.filename_prefix,
            self.filename_suffix,
            timeStamp
        );

        let stdout = ConsoleAppender::builder().build();

        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build(logFileName).unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Info)).unwrap();

        log4rs::init_config(config).unwrap();
    }
}
