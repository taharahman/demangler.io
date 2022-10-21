use regex::Regex;
use msvc_demangler;
use cpp_demangle::Symbol;
use std::string::ToString;

pub fn demangle(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\s]+").unwrap();
    }

    let mut input = String::from(text);
    for m in RE.find_iter(text).collect::<Vec<_>>().iter().rev() {
        let mut demangled: String = String::from(m.as_str());

        if let Ok(sym) = msvc_demangler::demangle(&demangled, msvc_demangler::DemangleFlags::llvm()) {
            demangled = sym.to_string();
        }
        if let Ok(sym) = Symbol::new(m.as_str()) {
            demangled = sym.to_string();
        }

        if demangled.eq(m.as_str()) { continue; }
        input.replace_range(m.start()..m.end(), demangled.as_ref());
    }
    return input;
}