fn main() {
    let status = std::process::Command::new("sccache")
        .arg("--start-server")
        .spawn()
        .expect("spawn sccache --start-server")
        .wait()
        .expect("wait sccache --start-server");

    if !status.success() {
        panic!("sccache --start-server failed");
    }

    std::thread::sleep(std::time::Duration::from_secs(9999));
}
