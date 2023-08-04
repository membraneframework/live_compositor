use std::error::Error;
use std::path::Path;
use std::{env, fs};

#[cfg(target_os = "linux")]
pub fn bundle_app(build_path: &Path) -> Result<(), Box<dyn Error>> {
    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent().unwrap();

    let _ = fs::remove_dir_all(current_dir.join("process_helper"));

    fs::copy(
        build_path.join("process_helper"),
        current_dir.join("process_helper"),
    )?;

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn bundle_app(build_path: &Path) -> Result<(), Box<dyn Error>> {
    use fs_extra::dir::{self, CopyOptions};

    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent().unwrap();
    let bundle_path = current_dir.join("video_compositor.app").join("Contents");

    let _ = fs::remove_dir_all(&bundle_path);

    for dir in ["MacOS", "Resources"] {
        fs::create_dir_all(bundle_path.join(dir))?;
    }

    dir::copy(
        build_path.join("Frameworks"),
        &bundle_path,
        &CopyOptions::default(),
    )?;

    let bundle_info = fs::read_to_string(build_path.join("resources").join("info.plist"))?
        .replace("${EXECUTABLE_NAME}", "Video Compositor");
    fs::write(
        bundle_path.parent().unwrap().join("Info.plist"),
        bundle_info,
    )?;

    let helper_info = fs::read_to_string(build_path.join("resources").join("helper-Info.plist"))?;
    let helpers = [
        ("video_compositor Helper", ""),
        ("video_compositor Helper (Alerts)", ".alerts"),
        ("video_compositor Helper (GPU)", ".gpu"),
        ("video_compositor Helper (Plugin)", ".plugin"),
        ("video_compositor Helper (Renderer)", ".renderer"),
    ];

    for (name, bundle_id) in helpers {
        bundle_helper(name, bundle_id, &helper_info, build_path, &bundle_path)?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn bundle_helper(
    name: &str,
    bundle_id: &str,
    info_data: &str,
    build_path: &Path,
    bundle_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let bundle_path = bundle_path
        .join("Frameworks")
        .join(format!("{name}.app"))
        .join("Contents");

    for dir in ["MacOS", "Resources"] {
        fs::create_dir_all(bundle_path.join(dir))?;
    }

    fs::copy(
        build_path.join("process_helper"),
        bundle_path.join("MacOS").join(name),
    )?;

    let info_data = info_data
        .replace("${EXECUTABLE_NAME}", name)
        .replace("${BUNDLE_ID_SUFFIX}", bundle_id);

    fs::write(bundle_path.parent().unwrap().join("Info.plist"), info_data)?;

    Ok(())
}
