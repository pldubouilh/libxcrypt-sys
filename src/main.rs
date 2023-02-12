use std::ffi::{CStr, CString};

use libxcrypt_sys::crypt;

fn main() {
    let password = "mypassword";
    let sha256_setting = "$5$";
    let salt = "rDxsrps6AeTwJLRK";
    let settings = format!("{sha256_setting}{salt}");

    let ret_str = unsafe {
        // put phrase to cstring called ret
        let csetting = CString::new(settings).unwrap();
        let cpassword = CString::new(password).unwrap();
        let ret = crypt(cpassword.as_ptr(), csetting.as_ptr());
        let ret_cstr = CStr::from_ptr(ret);
        ret_cstr.to_str().unwrap()
    };

    // mkpasswd -m sha256crypt "mypassword"
    // $5$rDxsrps6AeTwJLRK$CHafsXkpg6bi5Z.kdTYhlWzmhqe9Q.RRPm0LWi/bckC

    let ret_assumed = format!(
        "{}{}${}",
        sha256_setting, salt, "CHafsXkpg6bi5Z.kdTYhlWzmhqe9Q.RRPm0LWi/bckC"
    );
    assert!(ret_str == ret_assumed);

    println!("ret_str {ret_str:?}");
}
