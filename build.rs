use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
extern crate os_info;

static WIT_FILES: &'static [&str] = &[
    "wasmedge_ffmpeg_avcodec.wit",
    "wasmedge_ffmpeg_avdevice.wit",
    "wasmedge_ffmpeg_avfilter.wit",
    "wasmedge_ffmpeg_avformat.wit",
    "wasmedge_ffmpeg_avutil.wit",
    "wasmedge_ffmpeg_swscale.wit",
    "wasmedge_ffmpeg_swresample.wit"
];

fn program_exists(program: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(":") {
            let p_str = format!("{}/{}", path, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn main() {

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let witc_in_path = program_exists("witc");

    let exe = if !witc_in_path {
        let info = os_info::get();
        let exe = match info.os_type() {
            os_info::Type::Ubuntu => "witc-v0.4-ubuntu",
            os_info::Type::Macos => "witc-v0.4-macos",
            os_info::Type::Windows => "witc-v0.4-windows.exe",
            _ => panic!("Unsupported OS type"),
        };

        println!(
            "Downloading {}",
            format!(
                "https://github.com/second-state/witc/releases/download/v0.4/{}",
                exe
            )
        );

        match info.os_type() {
            os_info::Type::Ubuntu | os_info::Type::Macos => {
                Command::new("wget")
                    .arg(format!(
                        "https://github.com/second-state/witc/releases/download/v0.4/{}",
                        exe
                    ))
                    .output()
                    .expect("Failed to get executable");
            }
            os_info::Type::Windows => {
                let cmd = format!(
                    "Invoke-WebRequest -URI {} -OutFile {}",
                    format!(
                        "https://github.com/second-state/witc/releases/download/v0.4/{}",
                        exe
                    ),
                    exe
                );
                powershell_script::run(&cmd).expect("Failed to get executable");
            }
            _ => panic!("Unsupported OS type"),
        };

        Command::new("chmod")
            .arg("+x")
            .arg(exe)
            .output()
            .expect("Failed to change mode for downloaded executable");

        format!("./{}", exe)
    } else {
        "witc".to_string()
    };


    for wit_file in WIT_FILES {

        // witc plugin wasmedge_opencvmini.wit > src/generated.rs
        let output = Command::new(&exe)
            .arg("plugin")
            .arg(format!("witc/{}",wit_file))
            .output()
            .expect("Failed to execute command");

        println!("status: {}", output.status);

        let wit_file = wit_file.replace("wasmedge_ffmpeg_","").replace(".wit","");
        let dest_path = Path::new(&out_dir).join(format!("{}.rs",wit_file));
        fs::write(&dest_path, String::from_utf8_lossy(&output.stdout).as_ref()).unwrap();

    }

    if !witc_in_path {
        let _ = Command::new("rm")
            .arg(&exe)
            .output()
            .expect("Failed to get script");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
