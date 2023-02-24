use dll_syringe::{Syringe, process::OwnedProcess};
use std::{thread, time, env};

fn main() {
    // find target process by name
    let target_process =
        OwnedProcess::find_first_by_name("notepad.exe").unwrap();

    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);

    println!("Current directory: {}", std::env::current_dir().unwrap().display());

    // inject the payload into the target process
    // let injected_payload = syringe.inject("hello_inject\\target\\debug\\deps\\hello.dll").unwrap();
    // let injected_payload = syringe.inject("target\\debug\\deps\\hello.dll").unwrap();
    let injected_payload = syringe.inject("..\\target\\debug\\deps\\hello.dll").unwrap();
    println!("DLL injected successfully!");

    // do something else
    let ten_millis = time::Duration::from_secs(60);

    println!("Sleeping for 30 secs...");
    thread::sleep(ten_millis);

    // eject the payload from the target (optional)
    syringe.eject(injected_payload).unwrap();
}