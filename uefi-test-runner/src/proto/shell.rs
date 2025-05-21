use uefi::boot;
use uefi::proto::shell::{Shell, ShellFileHandle};
use uefi::CStr16;
use uefi::data_types::Char16;

pub fn test() {
    info!("Running shell protocol tests");

    let handle = boot::get_handle_for_protocol::<Shell>().expect("No Shell handles");

    let mut shell =
        boot::open_protocol_exclusive::<Shell>(handle).expect("Failed to open Shell protocol");

    // create some files
    // let mut test_buf = [0u16; 12];
    // let test_str = CStr16::from_str_with_buf("test", &mut test_buf).unwrap();

    /* Test retrieving list of environment variable names (null input) */
    let cur_env_vec = shell.get_env(None).expect("Could not get environment variable").vec().unwrap();
    let mut test_buf = [0u16; 64];
    assert_eq!(*cur_env_vec.get(0).unwrap(), CStr16::from_str_with_buf("path", &mut test_buf).unwrap());
    assert_eq!(*cur_env_vec.get(1).unwrap(), CStr16::from_str_with_buf("nonesting", &mut test_buf).unwrap());

    // Debug statements TODO: Remove
    info!("cur_env_vec size: {}", cur_env_vec.len());
    for (i, env_var) in cur_env_vec.iter().enumerate() {
        info!("i: {}, env_var: {}", i, env_var);
    }

    /* Test setting and getting a specific environment variable */
    let mut test_env_buf = [0u16; 32];
    let test_var = CStr16::from_str_with_buf("test_var", &mut test_env_buf).unwrap();
    let mut test_val_buf = [0u16; 32];
    let test_val = CStr16::from_str_with_buf("test_val", &mut test_val_buf).unwrap();
    assert!(shell.get_env(Some(test_var)).is_none());
    shell.set_env(test_var, test_val, false);
    let cur_env_str = shell.get_env(Some(test_var)).expect("Could not get environment variable").val().unwrap();
    assert_eq!(cur_env_str, test_val);

    // let mut cur_fs_buf = [0u16; 32];
    // let cur_fs_str = CStr16::from_str_with_buf("", &mut cur_fs_buf).unwrap();
    // info!("cur_fs_str size 1: {}", cur_fs_str.num_chars());


    // let cur_fs_str = shell.get_cur_dir(None).expect("Could not get the current file system mapping");
    // info!("cur_fs_str size: {}", cur_fs_str.num_chars());
    // info!("cur_fs_str: {}", cur_fs_str);


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
