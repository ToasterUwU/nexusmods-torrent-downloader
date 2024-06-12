use notify_rust::Notification;
use reqwest;
use std::env;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::exit;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            register_nxm_handler();
            exit(0)
        }
        2 => println!("NXM Link: {}", args[1]),
        _ => {
            notify("Invalid number of arguments provided");
            exit(1);
        }
    }

    if args[1].starts_with("nxm://") == false {
        notify("Not a valid NXM link");
        exit(1);
    }
    let without_protocol = args[1].replace("nxm://", "");
    let without_http_get_args = without_protocol
        .split("?")
        .next()
        .unwrap_or(&without_protocol);
    let parts: Vec<&str> = without_http_get_args.split("/").collect();

    if parts.len() != 5 {
        notify("Not a valid NXM link");
        exit(1);
    }

    let game = parts[0];
    let section = parts[1];
    let item_id = parts[2];
    let _ = parts[3]; // Ignoring the unused variable
    let file_id = parts[4];

    let common_url = format!("https://raw.githubusercontent.com/ToasterUwU/nexusmods-torrent-downloader/main/torrents/files/{}/{}/{}/{}.", game, section, item_id, file_id);
    let torrent_url = common_url.clone() + "torrent";
    let name_url = common_url + "name";

    let response = reqwest::blocking::get(&name_url).unwrap();

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Read the response body as a string
        let name = response.text().unwrap();

        let response = reqwest::blocking::get(torrent_url).unwrap();
        let download_folder = dirs::download_dir().unwrap();
        let file_path = download_folder.join(name.clone() + ".torrent");

        // Create a file at the specified path
        let mut file = File::create(&file_path).unwrap();

        // Copy the response body to the file
        copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();

        notify(format!("File downloaded and saved as: {}", name).as_str());

        // Now you can save the content to a variable or process it further
    } else if response.status() == reqwest::StatusCode::NOT_FOUND {
        notify("This File isnt in the Torrent list yet. Please use the traditional way of downloading it.")
    } else {
        // Print an error message if the request was not successful
        notify(format!("Request failed with status code: {}", response.status()).as_str());
        exit(1);
    }
}

// nxm://skyrimspecialedition/mods/266/files/491975?key=Nb09aEJBCRFfLmPRepJloA&expires=1718354667&user_id=183623037
// https://github.com/ToasterUwU/nexusmods-torrent-downloader/raw/main/torrents/files/skyrimspecialedition/mods/266/491975/Unofficial%20Skyrim%20Special%20Edition%20Patch-266-4-3-1-1713394824.7z.torrent
#[cfg(target_os = "windows")]
fn register_nxm_handler() {
    let registry_script = format!(
        include_str!("../assets/register_nxm_protocol.reg");
        std::env::current_exe().unwrap().to_string_lossy()
    );

    fs::write("register_nxm_protocol.reg", registry_script).expect("Unable to write file");
    Command::new("regedit")
        .args(&["/s", "register_nxm_protocol.reg"])
        .status()
        .expect("Failed to execute regedit");
}

#[cfg(target_os = "linux")]
fn register_nxm_handler() {
    let home_dir = env::var("HOME").expect("HOME environment variable not found");
    let desktop_file_path = format!(
        "{}/.local/share/applications/nexusmods-torrent-downloader.desktop",
        home_dir
    );
    let parent_dir = Path::new(&desktop_file_path)
        .parent()
        .expect("Unable to get parent directory");
    fs::create_dir_all(parent_dir).expect("Unable to create directory");

    let desktop_entry = include_str!("../assets/nexusmods-torrent-downloader.desktop");
    fs::write(&desktop_file_path, desktop_entry).expect("Unable to write file");

    // Update the MIME database
    Command::new("update-mime-database")
        .status()
        .expect("Failed to update MIME database");

    // Set the default handler
    Command::new("xdg-mime")
        .args(&["default", &desktop_file_path, "x-scheme-handler/nxm"])
        .status()
        .expect("Failed to set default handler");
}

fn notify(msg: &str) {
    println!("{}", msg);
    Notification::new()
        .summary("NexusMods Torrent Downloader")
        .body(msg)
        .show()
        .unwrap();
}
