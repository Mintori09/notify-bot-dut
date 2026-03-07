use crate::database::{config_dir, config_file_path};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf, process::Command};

// ── Top-level CLI ─────────────────────────────────────────────────────────────

/// DUT notify bot — forwards DUT notices to a Telegram group.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommand>,

    // ── Run-mode overrides (only used when no subcommand is given) ────────────

    /// Override bot token from config.json (env: TELOXIDE_TOKEN)
    #[arg(short, long, value_name = "TOKEN", env = "TELOXIDE_TOKEN", global = true)]
    pub token: Option<String>,

    /// Override Telegram chat/group ID from config.json (env: CHAT_ID)
    #[arg(short, long, value_name = "ID", env = "CHAT_ID", global = true)]
    pub chat_id: Option<i64>,

    /// Override class filter list (repeatable: -f 23.Nh16 -f 23.Nh44)
    #[arg(short, long, value_name = "PATTERN", global = true)]
    pub filter: Vec<String>,
}

// ── Subcommands ───────────────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Open config.json in $EDITOR (fallback: nano)
    Config,

    /// Generate and activate a systemd user service for the bot
    InstallService,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// Open config.json in the user's preferred editor.
pub fn open_config() {
    let path = config_file_path();

    // Create the file from template if it doesn't exist yet
    if !path.exists() {
        let dir = config_dir();
        fs::create_dir_all(&dir).expect("Failed to create config directory");
        let template = r#"{
  "teloxide_token": "",
  "chat_id": 0,
  "filter_notice": ["23.Nh16", "23.Nh44"]
}
"#;
        fs::write(&path, template).expect("Failed to write config template");
        println!("✓ Created config template at {}", path.display());
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    println!("Opening {} with {}", path.display(), editor);

    let status = Command::new(&editor)
        .arg(&path)
        .status()
        .unwrap_or_else(|_| panic!("Failed to launch editor: {}", editor));

    if !status.success() {
        eprintln!("Editor exited with non-zero status");
        std::process::exit(1);
    }
}

/// Generate ~/.config/systemd/user/notify-bot-dut.service and enable it.
pub fn install_service() {
    let binary = std::env::current_exe().expect("Cannot determine current executable path");
    let binary_str = binary.display().to_string();

    let service_dir = systemd_user_dir();
    fs::create_dir_all(&service_dir).expect("Failed to create systemd user directory");

    let service_path = service_dir.join("notify-bot-dut.service");
    let unit = format!(
        "[Unit]\n\
        Description=DUT Notify Bot\n\
        After=network-online.target\n\
        Wants=network-online.target\n\
        \n\
        [Service]\n\
        Type=simple\n\
        ExecStart={binary_str}\n\
        Restart=on-failure\n\
        RestartSec=30\n\
        \n\
        [Install]\n\
        WantedBy=default.target\n"
    );

    fs::write(&service_path, &unit).expect("Failed to write service file");
    println!("✓ Service file written to {}", service_path.display());

    run_systemctl(&["--user", "daemon-reload"]);
    run_systemctl(&["--user", "enable", "--now", "notify-bot-dut"]);
    println!("✓ Service enabled and started.");
    println!("  Use `systemctl --user status notify-bot-dut` to check status.");
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn systemd_user_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot find home directory")
        .join(".config/systemd/user")
}

fn run_systemctl(args: &[&str]) {
    let status = Command::new("systemctl")
        .args(args)
        .status()
        .expect("Failed to run systemctl");

    if !status.success() {
        eprintln!("systemctl {} failed", args.join(" "));
        std::process::exit(1);
    }
}
