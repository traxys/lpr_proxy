use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const PORT: u16 = 1468;

#[cfg_attr(feature = "structopt", derive(structopt::StructOpt))]
#[derive(Debug, Serialize, Deserialize)]
pub struct LprOptions {
    #[cfg_attr(feature = "structopt", structopt(short = "E"))]
    encrypt: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "H"))]
    server: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "C"))]
    name_c: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "J"))]
    name_j: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "T"))]
    name_t: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "P"))]
    printer: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "U"))]
    username: Option<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "#"))]
    copies: Option<u64>,
    #[cfg_attr(feature = "structopt", structopt(short = "h"))]
    no_banner: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "l"))]
    no_filtering: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "m"))]
    mail: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "o"))]
    options: Vec<String>,
    #[cfg_attr(feature = "structopt", structopt(short = "p"))]
    with_header: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "q"))]
    hold: bool,
    #[cfg_attr(feature = "structopt", structopt(short = "r"))]
    delete_after: bool,
}

impl LprOptions {
    pub fn to_options(self) -> Vec<Cow<'static, str>> {
        let mut args: Vec<Cow<_>> = Vec::new();
        for option in self.options {
            args.push("-o".into());
            args.push(option.into());
        }
        if self.no_banner {
            args.push("-h".into());
        }
        if self.no_filtering {
            args.push("-l".into());
        }
        if self.mail {
            args.push("-m".into());
        }
        if self.with_header {
            args.push("-p".into());
        }
        if self.hold {
            args.push("-q".into());
        }
        if self.delete_after {
            args.push("-r".into());
        }
        if let Some(copies) = self.copies {
            args.push("-#".into());
            args.push(format!("{}", copies).into());
        }
        if let Some(username) = self.username {
            args.push("-U".into());
            args.push(username.into());
        }
        if let Some(printer) = self.printer {
            args.push("-P".into());
            args.push(printer.into());
        }
        if self.encrypt {
            args.push("-E".into())
        }
        if let Some(server) = self.server {
            args.push("-H".into());
            args.push(server.into());
        }
        if let Some(name_c) = self.name_c {
            args.push("-C".into());
            args.push(name_c.into());
        }
        if let Some(name_j) = self.name_j {
            args.push("-J".into());
            args.push(name_j.into());
        }
        if let Some(name_t) = self.name_t {
            args.push("-T".into());
            args.push(name_t.into());
        }
        args
    }
}
