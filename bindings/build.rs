fn main() {
    windows::build!(
        windows::win32::menus_and_resources::{
            GetCursorPos,
            SetCursorPos,
        }
        windows::win32::display_devices::POINT
        windows::win32::ipc::CreateNamedPipeA
    );
}