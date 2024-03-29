use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
///Sets up console and file logger
pub fn setup() -> anyhow::Result<()> {
    let mut current_exe_dir =
        std::env::current_exe().context("Getting current exe location failed")?;
    current_exe_dir.pop();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[ {h({l})} | {d(%Y-%m-%d %H:%M:%S)} | {M}] {m}{n}",
        )))
        .build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[ {h({l})} | {d(%Y-%m-%d %H:%M:%S)} | {M}] {m}{n}",
        )))
        .build(current_exe_dir.join("logs.txt"))
        .context("Creating File Log Appender Failed")?;
    /* let database = DatabaseAppender::new(Box::new(PatternEncoder::new(
        "{l}[ {d(%Y-%m-%d %H:%M:%S)} | {M}] {m}",
    ))); */

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        /* .appender(Appender::builder().build("database", Box::new(database))) */
        .build(
            Root::builder()
                .appenders(["stdout", "file" /* , "database" */])
                .build(LevelFilter::Info),
        )
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();

    Ok(())
}
