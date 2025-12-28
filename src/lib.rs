use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

// Orijinal execve imzası
type ExecveFn = extern "C" fn(*const c_char, *const *const c_char, *const *const c_char) -> c_int;

#[no_mangle]
pub unsafe extern "C" fn execve(
    path: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    let path_ptr = if path.is_null() { return -1; } else { path };
    let path_str = CStr::from_ptr(path_ptr).to_string_lossy();
    
    // Orijinal sembolü bul (RTLD_NEXT kullanımı proot/preload için daha sağlıklıdır)
    let original_execve: ExecveFn = std::mem::transmute(libc::dlsym(
        libc::RTLD_NEXT,
        CString::new("execve").unwrap().as_ptr(),
    ));

    // KRİTİK: Hedef zaten emülatör değilse ve x86 binary çalıştırma niyetindeyse yönlendir
    // Burada "moltenemu" yolunu senin proot içindeki kurulumuna göre ayarla
    if !path_str.contains("moltenemu") && !path_str.contains("fex") {
        // Kaptan, buradaki yolu proot içindeki nihai konuma göre güncellemelisin
        let emulator_path = CString::new("/usr/bin/box64").unwrap();
        
        // Vibe Coding Notu: Argüman manipülasyonu gerekirse buraya ekleme yapacağız
        return original_execve(emulator_path.as_ptr(), argv, envp);
    }

    original_execve(path, argv, envp)
}
