use crate::PathBuf;
use std::process::Command;

pub trait PathBufEx {
    // 定义你想要添加的方法
    // 例如，一个方法将字符串添加到路径末尾
    fn open_folder(self);
    fn open_file(self,file_name :&str);
}


impl PathBufEx for std::path::PathBuf {

   

    #[cfg(target_os = "windows")]
    fn open_folder(self) {
        if self.is_dir() {
            let operation = "open\0";
            let file = self.to_str().unwrap();
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
        panic!("folder is not dir")
    }

    #[cfg(target_os = "linux")]
    fn open_folder(self) {
        if self.is_dir() {
            Command::new("xdg-open")
                .arg(self.to_str().unwrap())
                .spawn()
                .expect("Failed to Open Folder");
        }
        panic!("folder is not dir")
    }
    #[cfg(target_os = "macos")]
    fn open_folder(self) {
        if self.is_dir() {
            let output = Command::new("open")
                .arg(self.to_str().unwrap())
                .spawn()
                .expect("Failed to Open Folder");
        }
        panic!("folder is not dir")
    }

    #[cfg(target_os = "linux")]
    fn open_file(self,file_name :&str) {
        let mut file_dir = PathBuf::from(self);
        file_dir.push(file_name);
        if file_dir.is_file() {
             Command::new("xdg-open")
                .arg(file_dir.to_str().unwrap())
                .spawn()
                .expect("Failed to Open File");
        }
        panic!("folder is not file")
    }

    #[cfg(target_os = "windows")]
    fn open_file(self,file_name :&str) {
        let mut file_dir = PathBuf::from(self);
        file_dir.push(file_name);
        if file_dir.is_file() {
            let operation = "open\0";
            let file = file_name.to_str().unwrap();
            let parameters = std::ptr::null();
            let directory = std::ptr::null();
            let show_cmd = 1; // SW_SHOWNORMAL
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
        panic!("folder is not file")
    }

    #[cfg(target_os = "macos")]
    fn open_file(self,file_name :&str) {
        let mut file_dir = PathBuf::from(self);
        file_dir.push(file_name);
        if file_dir.is_file() {
            let output = Command::new("open")
            .arg(file_dir.to_str().unwrap())
            .output()
            .spawn("Failed to execute command");
        }
        panic!("folder is not file")
    }
}