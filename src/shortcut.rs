use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
#[link(name = "shell32")]
extern "system" {
    fn ShellExecuteA(
        hwnd: isize,
        lpOperation: *const i8,
        lpFile: *const i8,
        lpParameters: *const i8,
        lpDirectory: *const i8,
        nShowCmd: i32,
    ) -> isize;
}

pub fn open_folder(path : PathBuf) {
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path.to_str().unwrap())
            .spawn()
            .expect("Failed to Open Folder");
    }
    #[cfg(target_os = "windows")]
    {
        let operation = "open\0";
        let file = path.to_str().unwrap();
        let parameters = std::ptr::null();
        let directory = std::ptr::null();
        let show_cmd = 1;
        unsafe {
            ShellExecuteA(
                0,
                operation.as_ptr() as *const i8,
                file.as_ptr() as *const i8,
                parameters,
                directory,
                show_cmd,
            );
        }
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path.to_str().unwrap())
            .spawn()
            .expect("Failed to Open Folder");
    }
}