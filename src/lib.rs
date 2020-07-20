use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const PORT: u16 = 1468;

#[derive(Debug, Serialize, Deserialize, structopt::StructOpt)]
pub struct LprOptions {
    #[serde(default)]
    #[structopt(short = "E")]
    encrypt: bool,
    #[structopt(short = "H")]
    server: Option<String>,
    #[structopt(short = "C")]
    name_c: Option<String>,
    #[structopt(short = "J")]
    name_j: Option<String>,
    #[structopt(short = "T")]
    name_t: Option<String>,
    #[structopt(short = "P")]
    printer: Option<String>,
    #[structopt(short = "U")]
    username: Option<String>,
    #[structopt(short = "#")]
    copies: Option<u64>,
    #[serde(default)]
    #[structopt(short = "h")]
    no_banner: bool,
    #[serde(default)]
    #[structopt(short = "l")]
    no_filtering: bool,
    #[serde(default)]
    #[structopt(short = "m")]
    mail: bool,
    #[serde(default)]
    #[structopt(short = "o", number_of_values = 1)]
    options: Vec<String>,
    #[serde(default)]
    #[structopt(short = "p")]
    with_header: bool,
    #[serde(default)]
    #[structopt(short = "q")]
    hold: bool,
    #[serde(default)]
    #[structopt(short = "r")]
    delete_after: bool,
    files: Vec<std::path::PathBuf>,
}

impl LprOptions {
    pub fn truncate(&mut self, prefix: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let prefix = prefix.as_ref();
        for file in &mut self.files {
            let canon = file.canonicalize()?;
            canon.strip_prefix(prefix)?;
            *file = canon;
        }
        Ok(())
    }
    pub fn rebuild(&mut self, prefix: impl AsRef<std::path::Path>) {
        let prefix = prefix.as_ref();
        for file in &mut self.files {
            *file = prefix.join(&file);
        }
    }

    pub fn to_options(self) -> Vec<std::ffi::OsString> {
        let mut args: Vec<std::ffi::OsString> = Vec::new();
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
        for file in self.files {
            args.push(file.into());
        }
        args
    }
}
