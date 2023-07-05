use std::env;
use std::ffi::OsString;
use std::process::{self, Command, Stdio};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, env = "FLY_APP_NAME")]
    app: String,

    #[command(subcommand)]
    subcmd: SubCmd,
}

#[derive(Parser, Debug, Clone)]
enum SubCmd {
    Up,
    Down,
    Exec(ExecCmd),
}

#[derive(Parser, Debug, Clone)]
struct ExecCmd {
    command: OsString,
    args: Vec<OsString>,
}

fn fly_auth_token() -> String {
    if let Ok(token) = env::var("FLY_AUTH_TOKEN") {
        return token;
    }

    match invoke_flyctl() {
        Ok(token) => {
            return token;
        }
        Err(e) => {
            eprintln!("fly-sccache: FLY_AUTH_TOKEN not set, and failed to execute `flyctl auth token`: {:?}", e);
            process::exit(1);
        }
    }

    fn invoke_flyctl() -> anyhow::Result<String> {
        let output = Command::new("flyctl")
            .arg("auth")
            .arg("token")
            .stderr(Stdio::inherit())
            .output()?;

        if !output.status.success() {
            anyhow::bail!("exit status {}", output.status);
        }

        Ok(std::str::from_utf8(&output.stdout)?.trim().to_owned())
    }
}

fn main() {
    let args = Args::parse();
    let auth_token = fly_auth_token();
}
