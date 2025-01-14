#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
extern "system" {
    #[doc = "*Required features: 'Win32_System_Mailslots', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMailslotA(lpname: super::super::Foundation::PSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: 'Win32_System_Mailslots', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMailslotW(lpname: super::super::Foundation::PWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: 'Win32_System_Mailslots', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_Mailslots', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> super::super::Foundation::BOOL;
}
