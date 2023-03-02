use std::ffi::OsStr;

use anyhow::Result;

use headless_chrome::{Browser, LaunchOptions};

struct ElementCall {
    url: String,
    room: String,
    homeserver: String,
}

struct User {
    name: String,
    password: String,
}

fn join_call(ec: &ElementCall, user: &User) -> Result<()> {
    let launch_options = LaunchOptions {
        headless: false,
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
    };
    let browser = Browser::new(launch_options)?;

    let context = browser.new_context()?;
    let tab = context.new_tab()?;
    tab.navigate_to(&ec.url)?;

    let element = tab.wait_for_element("input#callName")?;
    element.type_into(&ec.room)?;
    element.click()?;

    tab.wait_for_element("input#displayName")?.click()?;
    tab.type_str(&user.name)?;
    tab.press_key("Enter")?;
    tab.wait_until_navigated()?;

    tab.wait_for_elements("button")?
        .into_iter()
        .find(|e| e.value.contains("Join call now"))
        .unwrap()
        .click()?;

    // Sleep for 20 seconds to allow the call to connect.
    std::thread::sleep(std::time::Duration::from_secs(20));

    Ok(())
}

fn main() {
    let ec = ElementCall {
        // url: "https://pr804--element-call.netlify.app".to_string(),
        url: "https://element-call.netlify.app".to_string(),
        room: "dcall".to_string(),
        homeserver: "call.ems.org".to_string(),
    };

    let user = User {
        name: "Hans Zimmer".to_string(),
        password: "123456".to_string(),
    };

    join_call(&ec, &user).unwrap();
}
