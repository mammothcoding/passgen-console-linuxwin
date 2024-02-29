extern crate windows_exe_info;
fn main() {
    windows_exe_info::icon::icon_ico(std::path::Path::new("icon.ico"));
    windows_exe_info::versioninfo::link_cargo_env();
}
