use std::collections::HashMap;
use std::ffi::CStr;
use xcb::xproto;

pub type XmodmapPke = HashMap<u8, Vec<String>>;

pub fn xmodmap_pke(conn: &xcb::Connection) -> Result<XmodmapPke, anyhow::Error> {
    let setup = conn.get_setup();
    let length = setup.max_keycode() - setup.min_keycode() + 1;
    let keyboard_mapping =
        xproto::get_keyboard_mapping(&conn, setup.min_keycode(), length).get_reply()?;
    let keysyms = keyboard_mapping.keysyms();
    let keysyms_per_keycode = keyboard_mapping.keysyms_per_keycode();
    let ptr_value = unsafe { &*(keyboard_mapping.ptr) };
    let n_keycodes = ptr_value.length / keysyms_per_keycode as u32;
    let mut result = HashMap::new();
    for keycode_idx in 0..n_keycodes {
        let mut syms = vec![];
        for keysym_idx in 0..keysyms_per_keycode {
            let sym =
                keysyms[keysym_idx as usize + keycode_idx as usize * keysyms_per_keycode as usize];
            if sym != 0 {
                let string_ptr = unsafe { x11::xlib::XKeysymToString(sym as u64) };
                if !string_ptr.is_null() {
                    let str: String = unsafe { CStr::from_ptr(string_ptr) }
                        .to_str()
                        .unwrap()
                        .to_owned();
                    syms.push(str);
                } else {
                    syms.push("NoSymbol".to_string());
                }
            };
        }
        result.insert(setup.min_keycode() + keycode_idx as u8, syms);
    }
    Ok(result)
}

pub fn print_xmodmap_pke(xmodmap_pke: &XmodmapPke) {
    for (key, values) in xmodmap_pke {
        print!("keycode {:3} =", key);
        for value in values {
            print!(" {}", value);
        }
        println!()
    }
}
