/// This enum holds the packets used by the server and client.
///
/// It is not sorted alphabetically, rather it is sorted according to the
/// Packet ID, which is 8 bits long, of Minecraft server protocol version 14
/// (the one used in Minecraft Beta 1.7.3).
///
/// More information on the protocol can be found on this helpful [wiki page]
/// (https://wiki.vg/Protocol&oldid=510).
pub enum Packet {
    /* 0x00 */ KeepAlive,
    /* 0x01 */ LoginRequest,
    /* 0x02 */ Handshake,
    /* 0x03 */ ChatMessage,
    /* 0x04 */ TimeUpdate,
    /* 0x05 */ EntityEquipment,
    /* 0x06 */ SpawnPosition,
    /* 0x07 */ UseEntity,
    /* 0x08 */ UpdateHealth,
    /* 0x09 */ Respawn,
    /* 0x0a */ PlayerFlying,
    /* 0x0b */ PlayerPosition,
    /* 0x0c */ PlayerLook,
    /* 0x0d */ PlayerPositionAndLook,
    /* 0x0e */ PlayerDigging,
    /* 0x0f */ PlayerBlockPlacement,
    /* 0x10 */ HoldingChange,
    /* 0x11 */ UseBed,
    /* 0x12 */ UseAnimation,
    /* 0x13 */ EntityAction,
    /* 0x14 */ NamedEntitySpawn,
    /* 0x15 */ PickupSpawn,
    /* 0x16 */ CollectItem,
    /* 0x17 */ AddObject,
    /* 0x18 */ MobSpawn,
    /* 0x19 */ AddPainting,

    /* 0x1a */ Unused_0x1A,

    /* 0x1b */ StanceUpdate,
    /* 0x1c */ EntityVelocity,
    /* 0x1d */ DestroyEntity,
    /* 0x1e */ EntityUnchanged,
    /* 0x1f */ EntityRelativeMove,
    /* 0x20 */ EntityLook,
    /* 0x21 */ EntityLookAndRelativeMove,
    /* 0x22 */ EntityTeleport,

    /* 0x23 */ Unused_0x23,
    /* 0x24 */ Unused_0x24,
    /* 0x25 */ Unused_0x25,

    /* 0x26 */ EntityStatus,
    /* 0x27 */ AttachEntity,
    /* 0x28 */ EntityMetadata,
    
    /* 0x29 */ Unused_0x29,
    /* 0x2a */ Unused_0x2A,
    /* 0x2b */ Unused_0x2B,
    /* 0x2c */ Unused_0x2C,
    /* 0x2d */ Unused_0x2D,
    /* 0x2e */ Unused_0x2E,
    /* 0x2f */ Unused_0x2F,
    /* 0x30 */ Unused_0x30,
    /* 0x21 */ Unused_0x21,
    
    /* 0x32 */ PreChunk,
    /* 0x33 */ MapChunk,
    /* 0x34 */ MultiBlockChange,
    /* 0x35 */ BlockChange,
    /* 0x36 */ BlockAction,
    
    /* 0x36 */ Unused_0x36,
    /* 0x37 */ Unused_0x37,
    /* 0x38 */ Unused_0x38,
    /* 0x39 */ Unused_0x39,
    /* 0x3a */ Unused_0x3A,
    /* 0x3b */ Unused_0x3B,
    
    /* 0x3c */ Explosion,
    /* 0x3d */ SoundEffect,

    // UNUSED (0x3E - 0x45 inclusive)

    /* 0x46 */ NewState,
    /* 0x47 */ Thunderbolt,

    // UNUSED (0x48 - 0x63 inclusive)

    /* 0x64 */ OpenWindow,
    /* 0x65 */ CloseWindow,
    /* 0x66 */ WindowClick,
    /* 0x67 */ SetSlot,
    /* 0x68 */ WindowItems,
    /* 0x69 */ UpdateProgressBar,
    /* 0x6a */ Transaction,

    /* 0x82 */ UpdateSign,
    /* 0x83 */ ItemData,
    /* 0x84 */ IncrementStatistic,

    /* 0xFF */ DisconnectOrKick,
}
