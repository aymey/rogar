use std::ffi::CString;

pub struct ReqSpawn {
    pub id: u8, // 0
    pub name: CString
}

pub struct ReqSpectate {
    pub id: u8 // 1
}

// TODO: one mouse move packet struct
pub struct ReqMouseMovei16 {
    pub id: u8, // 16
    pub x: u16,
    pub y: u16,
    // pub target: u32 // unused
}

pub struct ReqMouseMovei32 {
    pub id: u8, // 16
    pub x: u32,
    pub y: u32,
    // pub target: u32 // unused
}

pub struct ReqMouseMovef64 {
    pub id: u8, // 16
    pub x: f64,
    pub y: f64,
    // pub target: u32 // unused
}

pub struct ReqSplit {
    pub id: u8 // 17
}

pub struct ReqQPressed {
    pub id: u8 // 18
}

pub struct ReqQReleased {
    pub id: u8 // 19
}

pub struct ReqEjectMass {
    pub id: u8, // 21
}

pub struct ReqChatMessage {
    pub id: u8, // 99
    pub flags: u8, // = 0? ignore?
    pub content: CString
}

pub struct ResUpdateNodes {
    pub id: u8, // 16
    pub amount: u16,
    pub destruct: 1
}
