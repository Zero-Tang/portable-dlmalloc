use std::process::Command;
use std::env;

macro_rules! DDK_PATH
{
	() => ("V:\\Program Files\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.38.33130");
}

macro_rules! INC_PATH
{
	() => ("V:\\Program Files\\Windows Kits\\10\\Include\\10.0.26100.0");
}

fn main()
{
	let r_out_dir=env::var("OUT_DIR");
	if let Ok(out_dir)=r_out_dir
	{
		if cfg!(target_os="windows") && cfg!(target_arch="x86_64") && cfg!(target_env="msvc") && cfg!(target_vendor="pc")
		{
			Command::new(format!("{}\\bin\\Hostx64\\x64\\cl.exe",DDK_PATH!()))
									.args(&["../malloc.c",
											concat!("/I",DDK_PATH!(),"\\include"),
											concat!("/I",INC_PATH!(),"\\ucrt")
											,"/DPORTABLE","/DUSE_DL_PREFIX","/DNO_MALLOC_STATS=1","/DUSE_LOCKS=2",
											"/Zi","/nologo","/W3","/WX","/O2","/Zc:wchar_t","/FS","/FAcs","/MD",
											format!("/Fa{}\\malloc.cod",out_dir).as_str(),
											format!("/Fo{}\\malloc.obj",out_dir).as_str(),
											format!("/Fd{}\\vc140.pdb",out_dir).as_str(),
											"/GS-","/std:c17","/Qspectre","/TC","/c","/errorReport:queue"]).status().unwrap();
			Command::new(format!("{}\\bin\\Hostx64\\x64\\lib.exe",DDK_PATH!()))
									.args(&[format!("{}\\malloc.obj",out_dir).as_str(),"/NOLOGO",
											format!("/OUT:{}\\malloc.lib",out_dir).as_str(),
											"/Machine:X64","/ERRORREPORT:QUEUE"]).status().unwrap();
		}
		else
		{
			eprintln!("Targets other than \"x86_64-pc-windows-msvc\" is not supported yet!");
		}
		println!("cargo::rustc-link-search=native={}",out_dir);
		println!("cargo::rustc-link-lib=static=malloc");
		println!("cargo::rerun-if-changed=../malloc.c");
	}
}