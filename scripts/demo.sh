#!/usr/bin/env bash
# Demo script for context-builder v0.8.0 — records a clean asciinema demo
# Usage: asciinema rec --cols 100 --rows 30 --command="bash scripts/demo.sh" docs/demo.cast

set -e

# Simulate typing effect
type_cmd() {
    local cmd="$1"
    local delay="${2:-0.04}"
    printf '\033[1;32m❯\033[0m '
    for ((i=0; i<${#cmd}; i++)); do
        printf '%s' "${cmd:$i:1}"
        sleep "$delay"
    done
    sleep 0.4
    echo ""
}

# Section header
section() {
    echo ""
    printf '\033[1;35m━━━ %s ━━━\033[0m\n' "$1"
    sleep 0.8
}

# Set up a clean demo project in /tmp
DEMO_DIR=$(mktemp -d)
PROJECT="$DEMO_DIR/my-rust-app"
mkdir -p "$PROJECT/src" "$PROJECT/tests" "$PROJECT/docs"

cat > "$PROJECT/Cargo.toml" << 'EOF'
[package]
name = "my-rust-app"
version = "1.0.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
EOF

cat > "$PROJECT/src/main.rs" << 'EOF'
use std::io;
mod config;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::AppConfig::from_env()?;
    let server = handler::Server::new(config);
    server.run().await
}

fn parse_args() -> Vec<String> {
    std::env::args().collect()
}
EOF

cat > "$PROJECT/src/config.rs" << 'EOF'
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?,
            database_url: std::env::var("DATABASE_URL")?,
            max_connections: std::env::var("MAX_CONN").unwrap_or_else(|_| "10".into()).parse()?,
        })
    }

    pub fn is_production(&self) -> bool {
        self.host != "localhost" && self.host != "127.0.0.1"
    }

    pub fn connection_string(&self) -> String {
        format!("{}?max_connections={}", self.database_url, self.max_connections)
    }
}
EOF

cat > "$PROJECT/src/handler.rs" << 'EOF'
use crate::config::AppConfig;
use std::sync::Arc;

pub struct Server {
    config: Arc<AppConfig>,
}

impl Server {
    pub fn new(config: AppConfig) -> Self {
        Self { config: Arc::new(config) }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Listening on {}:{}", self.config.host, self.config.port);
        Ok(())
    }

    pub async fn health_check(&self) -> &'static str {
        "ok"
    }

    pub async fn handle_request(&self, path: &str) -> Result<String, String> {
        match path {
            "/" => Ok("Welcome!".into()),
            "/health" => Ok(self.health_check().await.into()),
            _ => Err("Not Found".into()),
        }
    }
}
EOF

cat > "$PROJECT/tests/test_server.rs" << 'EOF'
#[test]
fn test_config_default_port() {
    std::env::set_var("DATABASE_URL", "postgres://localhost/test");
    let config = my_rust_app::config::AppConfig::from_env().unwrap();
    assert_eq!(config.port, 8080);
}

#[test]
fn test_is_production() {
    let config = my_rust_app::config::AppConfig {
        host: "api.example.com".into(),
        port: 443,
        database_url: "postgres://prod/db".into(),
        max_connections: 50,
    };
    assert!(config.is_production());
}
EOF

cat > "$PROJECT/docs/README.md" << 'EOF'
# My Rust App

A high-performance async web server built with Tokio.

## Quick Start

```bash
export DATABASE_URL=postgres://localhost/mydb
cargo run
```
EOF

cd "$PROJECT"

clear
printf '\033[1;33m'
cat << 'BANNER'

   ██████╗ ██████╗ ███╗   ██╗████████╗███████╗██╗  ██╗████████╗
  ██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝
  ██║     ██║   ██║██╔██╗ ██║   ██║   █████╗   ╚███╔╝    ██║
  ██║     ██║   ██║██║╚██╗██║   ██║   ██╔══╝   ██╔██╗    ██║
  ╚██████╗╚██████╔╝██║ ╚████║   ██║   ███████╗██╔╝ ██╗   ██║
   ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝   ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝
BANNER
printf '\033[0m'
printf '\033[0;36m  ██████╗ ██╗   ██╗██╗██╗     ██████╗ ███████╗██████╗\033[0m\n'
printf '\033[0;36m  ██╔══██╗██║   ██║██║██║     ██╔══██╗██╔════╝██╔══██╗\033[0m\n'
printf '\033[0;36m  ██████╔╝██║   ██║██║██║     ██║  ██║█████╗  ██████╔╝\033[0m\n'
printf '\033[0;36m  ██╔══██╗██║   ██║██║██║     ██║  ██║██╔══╝  ██╔══██╗\033[0m\n'
printf '\033[0;36m  ██████╔╝╚██████╔╝██║███████╗██████╔╝███████╗██║  ██║\033[0m\n'
printf '\033[0;36m  ╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═╝\033[0m\n'
printf '\033[2m  v0.8.0 — LLM context from your codebase\033[0m\n'
sleep 2

# --- Demo 1: Preview ---
section "1. Preview what gets processed"
type_cmd "context-builder --preview -y"
context-builder --preview -y 2>/dev/null
sleep 1.5

# --- Demo 2: Full context generation ---
section "2. Generate full LLM context"
type_cmd "context-builder -o context.md -y"
context-builder -o context.md -y 2>/dev/null
sleep 1

type_cmd "head -30 context.md"
head -30 context.md
sleep 2.5

# --- Demo 3: Signatures (THE HEADLINE FEATURE) ---
section "3. NEW: Extract function signatures (tree-sitter)"
type_cmd "context-builder --signatures -o signatures.md -y"
context-builder --signatures -o signatures.md -y 2>/dev/null
sleep 1

type_cmd "cat signatures.md | head -60"
cat signatures.md | head -60
sleep 3

# --- Demo 4: Size comparison ---
section "4. Compare: full context vs signatures-only"
type_cmd "wc -l context.md signatures.md"
wc -l context.md signatures.md
sleep 2.5

# --- Demo 5: Structure ---
section "5. NEW: Structural summary"
type_cmd "context-builder --structure --signatures -f rs -o structured.md -y"
context-builder --structure --signatures -f rs -o structured.md -y 2>/dev/null
sleep 1

type_cmd "cat structured.md"
cat structured.md
sleep 3

echo ""
printf '\033[1;32m✨ Your codebase is now LLM-ready — with 70%% fewer tokens.\033[0m\n'
printf '\033[2m   cargo install context-builder --features tree-sitter-all\033[0m\n'
sleep 3

# Cleanup
rm -rf "$DEMO_DIR"
