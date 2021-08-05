fn main() {
    windows::build! {
        Windows::Win32::System::Services::OWNER_SECURITY_INFORMATION,
        Windows::Win32::Security::Authorization::SE_FILE_OBJECT
    };
}
