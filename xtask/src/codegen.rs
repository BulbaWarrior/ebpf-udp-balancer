use std::{fs::File, io::Write, path::PathBuf};

use aya_tool::generate::InputFile;

pub fn generate() -> Result<(), anyhow::Error> {
    let dir = PathBuf::from("demo-ebpf/src");
    let names: Vec<&str> = vec!["ethhdr", "iphdr", "udphdr"];
    let bindings = aya_tool::generate(
        InputFile::Btf(PathBuf::from("/sys/kernel/btf/vmlinux")),
        &names,
        &[],
    )?;
    let mut out = File::create(dir.join("bindings.rs"))?;
    write!(out, "{}", bindings)?;
    Ok(())
}
