use std::env;
use std::path::PathBuf;
use std::process::Command;

#[cfg(all(windows, target_env = "msvc"))]
pub fn is_msvc() -> bool {
  true
}

#[cfg(not(all(windows, target_env = "msvc")))]
pub fn is_msvc() -> bool {
  false
}

/// Runs a command, checks that it is successful, and
/// returns its output if requested
pub fn run_command(command: &mut Command, fetch_stdout: bool, pipe_output: bool) -> Result<String,String> {
  if pipe_output {
    command.stdout(std::process::Stdio::piped());
	command.stderr(std::process::Stdio::piped());
  }
  println!("Executing command: {:?}", command);

  // command.output() must be called before command.status()
  // to avoid freezes on Windows
  let output = if pipe_output || fetch_stdout {
    Some(try!(command.output().or(Err(format!("command execution failed: {:?}", command)))))
  } else {
    None
  };

  let status = try!(command.status().or(Err(format!("command execution failed: {:?}", command))));
  if status.success() {
    Ok(if let Some(output) = output {
	  if fetch_stdout {
	    try!(String::from_utf8(output.stdout).or(Err("comand output is not valid unicode")))
		} else {
		String::new()
		}
		} else {
		String::new()
		})
  } else {
    if let Some(output) = output {
	  use std::io::Write;
	  println!("Stdout:");
	  try!(std::io::stderr().write_all(&output.stdout).or(Err("output failed")));
	  println!("Stderr:");
	  try!(std::io::stderr().write_all(&output.stderr).or(Err("output failed")));
	}
	Err(format!("command failed with status {:?}: {:?}", status, command).into())
  }
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let mut c_lib_path = PathBuf::from(manifest_dir.clone());
	c_lib_path.push("c_lib");
	c_lib_path.push("install");
	c_lib_path.push("lib");
	println!("cargo:rustc-link-search={}", c_lib_path.to_str().unwrap());

    let mut build_path = PathBuf::from(manifest_dir.clone());
	build_path.push("c_lib");
	build_path.push("build");

    let mut source_path = PathBuf::from(manifest_dir.clone());
	source_path.push("c_lib");
	source_path.push("source");

    let mut install_path = PathBuf::from(manifest_dir);
	install_path.push("c_lib");
	install_path.push("install");

    //create directories
	::std::fs::create_dir_all(build_path.to_str().unwrap()).unwrap();
	::std::fs::create_dir_all(install_path.to_str().unwrap()).unwrap();

    //STEP1: run cmake
	let mut cmake_command = Command::new("cmake");
	cmake_command.arg(source_path.to_str().unwrap())
	             .arg(format!("-DCMAKE_INSTALL_PREFIX={}",install_path.to_str().unwrap()))
				 .current_dir(build_path.to_str().unwrap());
	if is_msvc() {
	    cmake_command.arg("-G").arg("NMake Makefiles");
		// Rust always links to release version of MSVC runtime, so
		// link will fail if C library is built in debug mode
		cmake_command.arg("-DCMAKE_BUILD_TYPE=Release");
	}

    run_command(&mut cmake_command, false, true).unwrap();

    //STEP2: make
	let make_command_name = if is_msvc() { "nmake" } else { "make" }.to_string();
	let mut make_args = Vec::new();
	if !is_msvc() {
	    make_args.push(format!("-j{}", 4));
	}
	make_args.push("install".to_string());
	let mut make_command = Command::new(make_command_name);
	make_command.args(&make_args)
	            .current_dir(build_path.to_str().unwrap());

/*if let Some(linker_env_library_dirs) = self.linker_env_library_dirs {
  if !linker_env_library_dirs.is_empty() {
    for name in &["LIBRARY_PATH", "LD_LIBRARY_PATH", "LIB"] {
	  let value = try!(add_env_path_item(name, (*linker_env_library_dirs).clone()));
	  make_command.env(name, value);
	}
  }
}*/
    run_command(&mut make_command, false, true).unwrap();
}
