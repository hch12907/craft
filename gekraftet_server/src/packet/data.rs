/// This enum represents the packets sent by the client to server and vice versa.
///
/// Under this protocol, the strings are UCS-2 (with the exception of OpenWindow)
/// and they're prefixed with a short denoting their length.
///
/// There are some changes between the vanilla packets and the packets shown here
/// with Y-axis expansion (from i8 to i32) being the most notable.
pub enum PacketData {
    KeepAlive,

    LoginRequest {
        id: i32,
        username: Box<str>,
        seed: u64,
        dimension: u8,
    },

    Handshake {
        username_or_hash: Box<str>,
    },

    ChatMessage {
        message: Box<str>,
    },

    TimeUpdate {
        ticks: i64,
    },

    EntityEquipment {
        entity_id: i32,
        slot: i16,
        item_id: i16,
        unknown: i16,
    },

    SpawnPosition {
        x: i32,
        y: i32,
        z: i32,
    },

    UseEntity {
        source_entity: i32,
        target_entity: i32,
        left_click: bool,
    },

    UpdateHealth {
        health: i16,
    },

    Respawn {
        dimension: i8,
    },

    PlayerFlying {
        on_ground: bool,
    },

    PlayerPosition {
        x: f64,
        y: f64,
        stance: f64,
        on_ground: bool,
    },

    PlayerLook {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },

    PlayerPositionAndLook {
        x: f64,
        y: f64,
        stance: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },

    PlayerDigging {
        status: u8, // TODO: use an actual enum for status
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
    },

    PlayerBlockPlacement {
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
        direction: i8,
        block_id: i16,
        amount: i8,
        damage: i16,
    },

    HoldingChange {
        slot_id: u16,
    },

    UseBed {
        player_id: i32,
        in_bed: u8,
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
    },

    UseAnimation {
        player_id: i32,
        animation: i8, // TODO: actual enum
    },

    EntityAction {
        entity_id: i32,
        action: i8, // TODO: actual enum
    },

    NamedEntitySpawn {
        entity_id: i32,
        name: Box<str>,
        x: i32,
        y: i32,
        z: i32,
        rotation: u8,
        pitch: u8,
        current_item: i16,
    },

    PickupSpawn {
        entity_id: i32,
        item: i16,
        count: i8,
        data: i16,
        x: i32,
        y: i32,
        z: i32,
        rotation: u8,
        pitch: u8,
        roll: u8,
    },

    CollectItem {
        collected_eid: i32,
        collector_eid: i32,
    },

    AddObject {
        entity_id: i32,
        object_type: i8, // TODO: actual enum
        x: i32,
        y: i32,
        z: i32,
        unknown_flag: i32,
        unknown_short1: i16,
        unknown_short2: i16,
        unknown_short3: i16,
    },

    MobSpawn {
        entity_id: i32,
        entity_type: i8, // TODO: actual enum
        x: i32,
        y: i32,
        z: i32,
        yaw: i8,
        pitch: i8,
        data_stream: Box<[u8]>, // TODO: actual metadata struct
    },

    AddPainting {
        entity_id: i32,
        title: Box<str>,
        x: i32,
        y: i32,
        z: i32,
        direction: i32,
    },

    StanceUpdate {
        // Unused?
        unknown_float1: f32,
        unknown_float2: f32,
        unknown_float3: f32,
        unknown_float4: f32,
        unknown_bool1: bool,
        unknown_bool2: bool,
    },

    EntityVelocity {
        entity_id: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },

    DestroyEntity {
        entity_id: i32,
    },

    EntityUnchanged {
        entity_id: i32,
    },

    EntityRelativeMove {
        entity_id: i32,
        dx: i8,
        dy: i8,
        dz: i8,
    },

    EntityLook {
        entity_id: i32,
        yaw: i8,
        pitch: i8,
    },

    EntityLookAndRelativeMove {
        entity_id: i32,
        dx: i8,
        dy: i8,
        dz: i8,
        yaw: i8,
        pitch: i8,
    },

    EntityTeleport {
        entity_id: i32,
        dx: i32,
        dy: i32,
        dz: i32,
        yaw: i8,
        pitch: i8,
    },

    EntityStatus {
        entity_id: i32,
        status: u8, // TODO: actual enum
    },

    AttachEntity {
        entity_id: i32,
        vehicle_id: u8,
    },

    EntityMetadata {
        entity_id: i32,
        metadata: Box<[u8]>, // TODO: actual metadata struct
    },

    PreChunk {
        x: i32,
        z: i32,
        init_chunk: bool
    },

    MapChunk {
        x: i32,
        y: i32, // NOTE: the vanilla game uses i16 here
        z: i32,
        size_x: i8,
        size_y: i8,
        size_z: i8,
        compressed_size: i32,
        compressed_data: Box<[u8]>,
    },

    MultiBlockChange {
        x: i32,
        z: i32,
        array_size: i16,
        coordinate_array: Box<[u16]>,
        type_array: Box<[u16]>,
        metadata_array: Box<[u16]>,
    },

    BlockChange {
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
        block_type: i8,
        block_metadata: i8,
    },

    BlockAction {
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
        states: [u8; 2],
    },

    Explosion {
        x: f64,
        y: f64,
        z: f64,
        radius: f32,
        record_count: i32,
        record: Box<[[u8; 3]]>,
    },

    SoundEffect {
        effect_id: i32,
        x: i32,
        y: i32, // NOTE: the vanilla game uses i8 here
        z: i32,
        sound_data: i32,
    },

    NewState {
        reason_code: u8,
    },

    Thunderbolt {
        entity_id: i32,
        unknown: bool,
        x: i32,
        y: i32,
        z: i32,
    },

    OpenWindow {
        window_id: i8,
        inventory_type: i8,
        window_title: Box<str>, // NOTE: this is the only UTF8 string here
        slots_number: u8,
    },

    CloseWindow {
        window_id: i8,
    },

    WindowClick {
        window_id: i8,
        clicked_slot: i16,
        right_clicked: bool,
        action_number: i16,
        shift_clicked: bool,
        item_id: i16,
        item_count: i8,
        item_uses: i16,
    },

    SetSlot {
        window_id: i8,
        update_slot: i16,
        item_id: i16,
        item_count: i8,
        item_uses: i16,
    },

    WindowItems {
        window_id: i8,
        item_count: i16,
        // stores (item ID, Some(count, uses)) if item ID != -1
        payload: Box<[(i16, Option<(i8, i16)>)]>,
    },

    UpdateProgressBar {
        window_id: i8,
        progress_bar: i16, // TODO: use an actual enum (type depends on block?)
        value: i16,
    },

    Transaction {
        window_id: i8,
        action_number: i16,
        accepted: bool,
    },

    UpdateSign {
        x: i32,
        y: i32, // NOTE: the vanilla game uses i16 here
        z: i32,
        text: Box<str>, // NOTE: max. 4 lines, and each line starts with its length
    },

    ItemData {
        item_type: i16,
        item_id: i16, // should be called damage value
        text_length: u8,
        text: Box<[u8]>,
    },

    IncrementStatistic {
        statistic_id: i32,
        amount: i8,
    },

    DisconnectOrKick {
        reason: Box<str>,
    },
}
