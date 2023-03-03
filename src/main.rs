use std::{ffi::OsStr, thread::sleep, time::Duration};

use anyhow::Result;
use structopt::StructOpt;

use headless_chrome::{browser::context::Context, Browser, LaunchOptions};

#[derive(Debug, StructOpt)]
struct CmdParameters {
    url: String,
    bots: usize,
    #[structopt(short, long)]
    headless: bool,
}

fn main() {
    let params = CmdParameters::from_args();

    let browser = Browser::new(LaunchOptions {
        headless: params.headless,
        sandbox: false,
        ignore_certificate_errors: true,
        args: vec![
            OsStr::new("--use-fake-ui-for-media-stream"),
            OsStr::new("--use-fake-device-for-media-stream"),
            OsStr::new("--disable-web-security"),
            OsStr::new("--allow-running-insecure-content"),
            OsStr::new("--unsafely-treat-insecure-origin-as-secure"),
            OsStr::new("--ignore-certificate-errors"),
            OsStr::new("--autoplay-policy=no-user-gesture-required"),
        ],
        ..LaunchOptions::default()
    })
    .expect("☠️ Failed to create a browser");

    let _bots = (0..params.bots)
        .map(|i| {
            println!("🚀 Launching bot {i}...");
            sleep(Duration::from_secs(3)); // Wait for 3 seconds before spawning to workaround the rate limiting.
            let bot = launch_bot(&browser, &params.url, format!("bot_{i}"))?;
            println!("✅ Bot {i} launched!");
            Ok(bot)
        })
        .collect::<Result<Vec<_>>>()
        .expect("☠️ Failed to launch all bots");

    println!("🎉 All bots are launched! Bots will leave the conference automatically after 5 minutes");
    println!("Press CTRL+C to stop immediately.");

    // Wait for 5 minutes before quitting.
    sleep(Duration::from_secs(5 * 60));
}

fn launch_bot<'a>(browser: &'a Browser, url: &str, name: String) -> Result<Context<'a>> {
    let context = browser.new_context()?;
    let tab = context.new_tab()?;
    tab.set_default_timeout(Duration::from_secs(3));

    tab.navigate_to(url)?;

    let element = tab.wait_for_element("input#displayName")?;
    element.type_into(&name)?;
    element.click()?;
    tab.press_key("Enter")?;
    tab.wait_until_navigated()?;
    tab.press_key("Enter")?;
    sleep(Duration::from_secs(2));
    tab.press_key("Enter")?;

    Ok(context)
}
