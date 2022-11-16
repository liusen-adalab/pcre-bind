use std::{ops::Range, process::Command, thread, time::Duration};

// extern crate libc;
mod bindings;

trait Find {
    fn pcre_find<S: AsRef<str>>(&self, pattern: S) -> Result<Option<Range<usize>>, ()>;
}

use crate::bindings::*;

use std::ptr;
impl<T: AsRef<str>> Find for T {
    fn pcre_find<S: AsRef<str>>(&self, pattern: S) -> Result<Option<Range<usize>>, ()> {
        let mut error_code = 0;
        let mut error_offset = 0;
        let pattern: &str = pattern.as_ref();
        let subject: &str = &self.as_ref();
        let code = unsafe {
            pcre2_compile_8(
                &*pattern.as_ptr(),
                pattern.len(),
                PCRE2_UCP | PCRE2_UTF,
                &mut error_code,
                &mut error_offset,
                ptr::null_mut(),
            )
        };
        let match_data = unsafe { pcre2_match_data_create_from_pattern_8(code, ptr::null_mut()) };
        let ovector = unsafe { pcre2_get_ovector_pointer_8(match_data) };
        let rc = unsafe {
            pcre2_match_8(
                code,
                subject.as_ptr(),
                subject.len(),
                0,
                0,
                match_data,
                ptr::null_mut(),
            )
        };
        if rc == -1 {
            return Ok(None);
        }
        let (s, e) = unsafe { (*ovector.offset(0), *ovector.offset(1)) };

        Ok(Some(s..e))
    }
}

fn send_to_server(content: impl AsRef<[u8]>) {
    use std::net::UdpSocket;

    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:56789")
        .expect("connect function failed");
    socket
        .send(content.as_ref())
        .expect("couldn't send message");
}

const SERVER_PORT_PCRE: u16 = 56789;

fn start_server() {
    println!("starting udp server");
    Command::new("bash")
        .env("SERVER_PORT_PCRE", SERVER_PORT_PCRE.to_string())
        .args(["./listener.sh"])
        .status()
        .unwrap();
}

fn main() {
    thread::spawn(move || {
        start_server();
    });
    thread::sleep(Duration::from_secs(1));

    let subject ="a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let pattern = r"(?<=\d{4})[\x{21}-\x{2F}|\x{3A}-\x{7E}|\x{7F}-\x{FFFF}]{3,11}(?=\S)";
    let find_res = subject.pcre_find(pattern);

    match find_res {
        Ok(Some(range)) => {
            // println!("match found: {}", &subject[range.clone()]);
            send_to_server(&subject[range]);
            thread::sleep(Duration::from_secs(1));
            println!();
            println!("matched content sended");
        }
        Ok(None) => {
            println!("not match");
        }
        Err(_) => {
            println!("err")
        }
    }
}
