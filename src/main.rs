use std::ffi::CString;
use std::net::UdpSocket;
use std::os::raw::c_char;

// Usage: https://www.pcre.org/current/doc/html/pcre2demo.html
#[link(name = "search", kind = "static")]
extern "C" {
    fn search(pat: *const c_char, subject: *const c_char, output: *mut u8) -> i32;
}

fn main() {
    let subject_rs = r"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let subject = CString::new(subject_rs).unwrap();
    let pattern = CString::new(r"\d{4}([^\s\d]{3,11})[^\s]").unwrap();
    let mut output: Vec<u8> = vec![0; subject_rs.len()];
    unsafe {
        let r = search(pattern.as_ptr(), subject.as_ptr(), output.as_mut_ptr());
        if r != 0 {
            return;
        }
    }
    let results = std::str::from_utf8(&output)
        .unwrap()
        .trim_matches(char::from(0))
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string() + "\n")
        .collect::<Vec<String>>();
    let addr = "127.0.0.1:34255";
    let socket = UdpSocket::bind(addr).unwrap();
    for r in results {
        socket.send_to(r.as_bytes(), "127.0.0.1:34254").unwrap();
    }
}
