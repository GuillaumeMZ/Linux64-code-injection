use std::ffi::CString;

use nix::unistd::Pid;

mod injector;
mod shellcodes;

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
compile_error!("Error: this injector only works on x86_64 Linux.");

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let target = Pid::from_raw(args[1].parse()?);
    let dlopen_address = u64::from_str_radix(&args[2], 16)?;
    //let dlclose_address = u64::from_str_radix(&args[3], 16)?;
    let dl_path = CString::new(&*args[4])?;
    let r#where = u64::from_str_radix(&args[5], 16)?;

    let injection_shellcode = shellcodes::assemble_injection_shellcode(&dl_path, dlopen_address)?;
    injector::inject_and_run_shellcode(&injection_shellcode, target, r#where)?;

    Ok(())
}
