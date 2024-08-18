use std::ffi::CString;

// TODO: should use `#[repr(packed)]`?

pub struct ReqProtocolVersion {
    pub opcode: u8,     // 0x01
    pub revision: u32,  // 0x00000003
}

pub struct ReqPing {
    pub opcode: u8  // 0x02
}

pub struct ReqUpdate {
    pub opcode: u8, // 0x03
    pub mposx: i32,
    pub mposy: i32,
    pub splits: u8,
    pub msplits: u8,
    pub ctrlf: ReqControlFlags, // (u8)
    // TODO: should we use seperate structs or parse dynamically?
    // depends on `ctrlf`:
    // spwnname: CString,
    // message: ChatMessage
}

// TODO: can automate difference * 2
#[repr(u8)]
pub enum ReqControlFlags {
    SpawnName   = 0x01,
    Spectate    = 0x02,
    QPress      = 0x04,
    QRelease    = 0x08,
    WPress      = 0x10,
    RPress      = 0x20,
    TPress      = 0x40,
    Message     = 0x80,
}

pub struct ReqChatMessage {
    pub count: u8,
    pub messages: Vec<CString>
}

pub struct ResPong {
    pub opcode: u8  // 0x02
}

pub struct ResUpdate {
    pub opcode: u8, // 0x03
    pub glupdf: ResGlobalUpdateFlags // (u16)
}

#[repr(u16)]
pub enum ResGlobalUpdateFlags {
    SpectateViewArea    = 0x0001,
    WorldBorder         = 0x0002,
    ServerInformation   = 0x0004,
    WorldInformation    = 0x0008,
    IncomingChatMessages= 0x0010,
    Leaderboard         = 0x0020,
    Clear               = 0x0040,
    AddedCells          = 0x0080,
    UpdatedCells        = 0x0100,
    EatenCells          = 0x0200,
    RemovedCells        = 0x0400
}

pub struct ResSpectateViewArea {
    pub centerx: f32,
    pub centery: f32,
    pub scale: f32,
}

pub struct ResWolrdBorder {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

pub struct ResServerInformation {
    pub gamemode: ResGamemodeType, // (u8)
    pub version_major: u8,
    pub version_minor: u8,
    pub version_patch: u8
}

#[repr(u8)]
pub enum ResGamemodeType {
    FFA     = 0x00,
    Teams   = 0x01
}

pub struct ResWorldInformation {
    pub server_name: CString,
    pub gamemode_name: CString,
    pub load_time: f32, // (0..=1)
    pub uptime: u32,    // (seconds)
    pub connections: u16,
    pub bots: u16,
    pub players_alive: u16,
    pub spectators: u16
}

pub struct ResIncomingChatMessage {
    pub count: u16,
    pub messages: Vec<ChatMessage>
}

pub struct ChatMessage {
    pub name: CString,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub is_server: bool,
    pub content: CString
}

pub struct ResLeaderboardData {
    pub kind: LeaderboardKind, // (u8)
    // depends on `kind`:
    // ffa: FFABoardData,
    // pie: PieChartData,
    // text: TextBoardData
}

#[repr(u8)]
pub enum LeaderboardKind {
    FFA = 0x01,
    Pie = 0x02,
    Text= 0x03
}

pub struct FFABoardData {
    pub pos: u16, // terminates array if = 0x0000
    pub flags: LeaderboardItemFlags, // (u8)
    pub name: CString
}

#[repr(u8)]
pub enum LeaderboardItemFlags {
    Highlighted = 0x01,
    Me          = 0x02
}

pub struct PieChartData {
    pub length: u16,
    pub item_size: Vec<f32> // (0..=1)
}

pub struct TextBoardData {
    pub length: u16,
    pub text: Vec<CString>
}

pub struct ResAddedCells {
    pub id: u32,    // terminates array if = 0x00000000
    pub kind: CellKind, // (u8)
    pub posx: f32,
    pub posy: f32,
    pub size: u16,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub info: CellInfo, // (u8)
    // depends on `info`
    // name: CString,
    // skin: CString
}

#[repr(u8)]
pub enum CellKind {
    Player  = 0x00,
    Pellet  = 0x01,
    Virus   = 0x02,
    Ejected = 0x03,
    Mother  = 0x04
}

#[repr(u8)]
pub enum CellInfo {
    Owned   = 0x01,
    Name    = 0x02,
    Skin    = 0x04
}

pub struct ResUpdatedCells {
    pub id: u32,    // terminates array if = 0x00000000
    pub flag: UpdatedFlags, // (u8)
    // depends on `flag`
    // posx: f32,
    // posy: f32,
    // size: u16
    // r: u8
    // g: u8
    // b: u8
    // name: CString
    // skin: CString
}

#[repr(u8)]
pub enum UpdatedFlags {
    Position    = 0x01,
    Size        = 0x02,
    Color       = 0x04,
    Name        = 0x08,
    Skin        = 0x10,
}

pub struct ResEatenCells {
    pub victim: u32,    // terminates array if = 0x00000000
    pub killer: u32
}

pub struct ResRemovedCells {
    pub id: u32     // terminates array if = 0x00000000
}
