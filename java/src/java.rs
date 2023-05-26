use std::error::Error;
use std::path::PathBuf;
use clap::Parser;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
    where
        T: std::str::FromStr,
        T::Err: Error + Send + Sync + 'static,
        U: std::str::FromStr,
        U::Err: Error + Send + Sync + 'static
{
    let pos = s.find('=')
        .ok_or_else(|| format!("invalid key-value pair: no separator in '{}'", s))?;
    let key = s[..pos].parse()?;
    let value = s[pos + 1..].parse()?;
    return Ok((key, value))
}

#[derive(Parser, Debug)]
#[command(
    author = "UselessMnemonic <madrigal.christopher.j@gmail.com>",
    version = "0.1.0",
    about = "Simple Rusticano CLI"
)]
struct JavaOpt {
    #[arg(
        long = "classpath",
        name = "path-list",
        help = "A ; separated list of directories to search for class files"
    )]
    class_path: Option<String>,

    #[arg(
        long = "dry-run",
        help = "Create VM and load main class but do not execute main method",
        default_value_t = false
    )]
    dry_run: bool,

    #[arg(
        short = 'D',
        name = "key=value",
        help = "Sets a system property",
        value_parser = parse_key_val::<String, String>
    )]
    defines: Vec<(String, String)>,

    #[arg(
        name = "mainclass",
        help = "Path to the class that contains a main function",
        required = true
    )]
    main_class: Vec<PathBuf>
}

fn main() {
    let java_opt = JavaOpt::parse();
    println!("{:#?}", java_opt)
}
