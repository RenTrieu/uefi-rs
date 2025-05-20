use uefi::boot;
use uefi::proto::shell::{Shell, ShellFileHandle};
use uefi::CStr16;

pub fn test() {
    info!("Running shell protocol tests");

    let handle = boot::get_handle_for_protocol::<Shell>().expect("No Shell handles");
    // let mut fs = uefi::fs::FileSystem::new(sfs
    // let (fs_handle, mut sfs) = find_test_disk();

    let mut shell =
        boot::open_protocol_exclusive::<Shell>(handle).expect("Failed to open Shell protocol");

    // create some files
    let mut test_buf = [0u16; 12];
    let test_str = CStr16::from_str_with_buf("test", &mut test_buf).unwrap();

    let cur_env_str = shell.get_env(None).expect("Could not get environment variable");
    info!("cur_env_str size: {}", cur_env_str.num_chars());
    info!("cur_env_str: {}", cur_env_str);
    // for (i, c) in cur_env_str.iter().enumerate() {
    //     info!("cur_env_str: i: {}, c: {}", i, c);
    // }

    // let mut cur_fs_buf = [0u16; 32];
    // let cur_fs_str = CStr16::from_str_with_buf("", &mut cur_fs_buf).unwrap();
    // info!("cur_fs_str size 1: {}", cur_fs_str.num_chars());
    let cur_fs_str = shell.get_cur_dir(None).expect("Could not get the current file system mapping");
    info!("cur_fs_str size: {}", cur_fs_str.num_chars());
    info!("cur_fs_str: {}", cur_fs_str);
    // for (i, c) in cur_fs_str.iter().enumerate() {
    //     info!("cur_fs_str: i: {}, c: {}", i, c);
    // }

    // unsafe {
        // info!("cur_fs_str: {}", cur_fs_str);
        // let mut expected_fs_str_buf = [0u16; 32];
        // assert_eq!(cur_fs_str, CStr16::from_str_with_buf("", &mut expected_fs_str_buf).unwrap());
        // //
        // Create a file
        // let status = shell.create_file(test_str, 0).expect("Could not create file");
        // let mut size: u64 = 0;
        // shell.get_file_size(f_handle, &mut size);
        // assert_eq!(size, 0);
    // }

    // get file tree
    // let mut str_buf = [0u16; 12];
    // let str_str = CStr16::from_str_with_buf(r"fs0:\*", &mut str_buf).unwrap();
    // let res = shell.find_files(str_str);
    // let list = res.unwrap();
    // let list = list.unwrap();
    // let first = list.first();

    info!("filetree test successful")
}
