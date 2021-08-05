mod bindings {
    windows::include_bindings!();
}

fn main() {
    bindings::Windows::Win32::System::Services::OWNER_SECURITY_INFORMATION
}
