use std::process::{Command, Stdio};

fn excute_cmd(cmd: &str) -> String {
  let _cmd = Command::new(cmd)
    .stdout(Stdio::piped())
    .spawn()
    .expect(&format!("Failed to start [{}] command", &cmd).to_string());

  let output = _cmd.wait_with_output().expect("Failed to output");

  String::from_utf8_lossy(&output.stdout).to_string()
}

fn excute_cmd_with_args(cmd: &str, args: Vec<&str>) -> String {
  let _cmd = Command::new(cmd)
    .args(&args)
    .stdout(Stdio::piped())
    .spawn()
    .expect(&format!("Failed to start [{}] command", &cmd).to_string());

  let output = _cmd.wait_with_output().expect("Failed to output");

  String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn pcr_read() -> String {
  excute_cmd("tpm2_pcrread")
}

pub fn readclock() -> String {
  excute_cmd("tpm2_readclock")
}


#[cfg(test)]
mod test {
  use std::process::{Command, Stdio};
  use super::*;

  #[test]
  fn run_cmd(){
    let rs = excute_cmd("pwd");

    println!("pwd => {}", rs);
  }

  #[test]
  fn run_cmd_with_args(){
    let rs = excute_cmd_with_args("ls", vec!["-a", "-l"]);

    println!("ls => {}", rs);
  }


}