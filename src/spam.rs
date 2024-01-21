use std::process::Command;

use anyhow::Context;

fn task(id: i32) {
    let output = Command::new("curl")
        .args(["localhost:3000"])
        .output()
        .with_context(|| format!("Failed to run curl in task {id}"))
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{id}: {stdout}");
}

fn main() -> anyhow::Result<()> {
    let mut handles = Vec::new();

    for i in 0..1000 {
        handles.push(std::thread::spawn(move || {
            task(i);
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    Ok(())
}
