#[repr(i16)]
pub enum ConVarType {
    Invalid = -1,
    Bool = 0,
    Int16 = 1,
    UInt16 = 2,
    Int32 = 3,
    UInt32 = 4,
    Int64 = 5,
    UInt64 = 6,
    Float32 = 7,
    Float64 = 8,
    String = 9,
    Color = 10,
    Vector2 = 11,
    Vector3 = 12,
    Vector4 = 13,
    QAngle = 14,
    Max = 15,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ConVarFlag: i32 {
        const NONE = 0;
        const UNREGISTERED = 1 << 0;
        const DEVELOPMENTONLY = 1 << 1;
        const GAME_DLL = 1 << 2;
        const CLIENT_DLL = 1 << 3;
        const HIDDEN = 1 << 4;
        const PROTECTED = 1 << 5;
        const SP_ONLY = 1 << 6;
        const ARCHIVE = 1 << 7;
        const NOTIFY = 1 << 8;
        const USER_INFO = 1 << 9;
        const CHEAT = 1 << 14;
        const PRINTABLE_ONLY = 1 << 10;
        const UNLOGGED = 1 << 11;
        const NEVER_AS_STRING = 1 << 12;
        const REPLICATED = 1 << 13;
        const DEMO = 1 << 16;
        const DONT_RECORD = 1 << 17;
        const RELOAD_MATERIALS = 1 << 20;
        const RELOAD_TEXTURES = 1 << 21;
        const NOT_CONNECTED = 1 << 22;
        const MATERIAL_SYSTEM_THREAD = 1 << 23;
        const ARCHIVE_XBOX = 1 << 24;
        const ACCESSIBLE_FROM_THREADS = 1 << 25;
        const SERVER_CAN_EXECUTE = 1 << 28;
        const SERVER_CANNOT_QUERY = 1 << 29;
        const CLIENT_CMD_CAN_EXECUTE = 1 << 30;
    }
}
