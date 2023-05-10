use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use tts::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TTS: Mutex<Tts> = Mutex::new(Tts::default().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn ttsrust_say(voice: usize, c_text: *const c_char) {
    let text = CStr::from_ptr(c_text).to_str().unwrap();
    let mut tts = TTS.lock().unwrap();
    let voices = tts.voices().unwrap();
    tts.set_voice(voices.get(voice).unwrap()).unwrap();
    tts.speak(text, false).unwrap();
}
