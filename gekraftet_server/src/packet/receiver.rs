use super::{ Metadata, PacketData };
use super::utils::*;
use tokio::io::{AsyncReadExt, Error as IoError, ErrorKind, Result as IoResult};

impl PacketData {
    pub(super) async fn read_keep_alive<I>(_input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt,
    {
        Ok(PacketData::KeepAlive)
    }

    pub(super) async fn read_login_request<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let id = input.read_i32().await?;
        let username = read_ucs2(input).await?;
        let seed = 0;
        input.read_u64().await?;
        let dimension = 0;
        input.read_u8().await?;

        Ok(PacketData::LoginRequest {
            id,
            username,
            seed,
            dimension,
        })
    }

    pub(super) async fn read_handshake<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt,
    {
        let username_or_hash = read_ucs2(input).await?;
        input.write_i8(2).await?;
        input.write_i16(1).await?;
        input.write_u16(45).await?;
        Ok(PacketData::Handshake { username_or_hash })
    }

    pub(super) async fn read_chat_message<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let message = read_ucs2(input).await?;
        Ok(PacketData::ChatMessage { message })
    }

    pub(super) async fn read_time_update<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let ticks = input.read_i64().await?;
        Ok(PacketData::TimeUpdate { ticks })
    }

    pub(super) async fn read_entity_equipment<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let slot = input.read_i16().await?;
        let item_id = input.read_i16().await?;
        let unknown = input.read_i16().await?;

        Ok(PacketData::EntityEquipment {
            entity_id,
            slot,
            item_id,
            unknown,
        })
    }

    pub(super) async fn read_spawn_position<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;

        Ok(PacketData::SpawnPosition { x, y, z })
    }

    pub(super) async fn read_use_entity<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let source_entity = input.read_i32().await?;
        let target_entity = input.read_i32().await?;
        let left_click = input.read_i8().await? == 1;

        Ok(PacketData::UseEntity {
            source_entity,
            target_entity,
            left_click,
        })
    }

    pub(super) async fn read_update_health<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let health = input.read_i16().await?;

        Ok(PacketData::UpdateHealth { health })
    }

    pub(super) async fn read_respawn<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let dimension = input.read_i8().await?;

        Ok(PacketData::Respawn { dimension })
    }

    pub(super) async fn read_player_flying<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let on_ground = input.read_i8().await? == 1;

        Ok(PacketData::PlayerFlying { on_ground })
    }

    pub(super) async fn read_player_position<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = f64::from_bits(input.read_u64().await?);
        let y = f64::from_bits(input.read_u64().await?);
        let stance = f64::from_bits(input.read_u64().await?);
        let z = f64::from_bits(input.read_u64().await?);
        let on_ground = input.read_i8().await? == 1;

        Ok(PacketData::PlayerPosition {
            x,
            y,
            stance,
            z,
            on_ground,
        })
    }

    pub(super) async fn read_player_look<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let yaw = f32::from_bits(input.read_u32().await?);
        let pitch = f32::from_bits(input.read_u32().await?);
        let on_ground = input.read_i8().await? == 1;

        Ok(PacketData::PlayerLook {
            yaw,
            pitch,
            on_ground,
        })
    }

    pub(super) async fn read_player_position_and_look<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = f64::from_bits(input.read_u64().await?);
        let y = f64::from_bits(input.read_u64().await?);
        let stance = f64::from_bits(input.read_u64().await?);
        let z = f64::from_bits(input.read_u64().await?);
        let yaw = f32::from_bits(input.read_u32().await?);
        let pitch = f32::from_bits(input.read_u32().await?);
        let on_ground = input.read_i8().await? == 1;

        Ok(PacketData::PlayerPositionAndLook {
            x,
            y,
            stance,
            z,
            yaw,
            pitch,
            on_ground,
        })
    }

    pub(super) async fn read_player_digging<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let status = input.read_u8().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;

        Ok(PacketData::PlayerDigging { status, x, y, z })
    }

    pub(super) async fn read_player_block_placement<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let direction = input.read_i8().await?;
        let block_id = input.read_i16().await?;
        let amount = input.read_i8().await?;
        let damage = input.read_i16().await?;

        Ok(PacketData::PlayerBlockPlacement {
            x,
            y,
            z,
            direction,
            block_id,
            amount,
            damage,
        })
    }

    pub(super) async fn read_holding_change<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let slot_id = input.read_u16().await?;

        Ok(PacketData::HoldingChange { slot_id })
    }

    pub(super) async fn read_use_bed<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let player_id = input.read_i32().await?;
        let in_bed = input.read_u8().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;

        Ok(PacketData::UseBed {
            player_id,
            in_bed,
            x,
            y,
            z,
        })
    }

    pub(super) async fn read_use_animation<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let player_id = input.read_i32().await?;
        let animation = input.read_i8().await?;

        Ok(PacketData::UseAnimation {
            player_id,
            animation,
        })
    }

    pub(super) async fn read_entity_action<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let action = input.read_i8().await?;

        Ok(PacketData::EntityAction {
            entity_id,
            action,
        })
    }

    pub(super) async fn read_named_entity_spawn<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let name = read_ucs2(input).await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let rotation = input.read_u8().await?;
        let pitch = input.read_u8().await?;
        let current_item = input.read_i16().await?;

        Ok(PacketData::NamedEntitySpawn {
            entity_id,
            name,
            x,
            y,
            z,
            rotation,
            pitch,
            current_item,
        })
    }

    pub(super) async fn read_pickup_spawn<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let item = input.read_i16().await?;
        let count = input.read_i8().await?;
        let data = input.read_i16().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let rotation = input.read_u8().await?;
        let pitch = input.read_u8().await?;
        let roll = input.read_u8().await?;
        

        Ok(PacketData::PickupSpawn {
            entity_id,
            item,
            count,
            data,
            x,
            y,
            z,
            rotation,
            pitch,
            roll,
        })
    }

    pub(super) async fn read_collect_item<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let collected_eid = input.read_i32().await?;
        let collector_eid = input.read_i32().await?;

        Ok(PacketData::CollectItem {
            collected_eid,
            collector_eid,
        })
    }

    pub(super) async fn read_add_object<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let object_type = input.read_i8().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let unknown_flag = input.read_i32().await?;
        let unknown_short1 = if unknown_flag > 0 { input.read_i16().await? } else { 0 };
        let unknown_short2 = if unknown_flag > 0 { input.read_i16().await? } else { 0 };
        let unknown_short3 = if unknown_flag > 0 { input.read_i16().await? } else { 0 };

        Ok(PacketData::AddObject {
            entity_id,
            object_type,
            x,
            y,
            z,
            unknown_flag,
            unknown_short1,
            unknown_short2,
            unknown_short3,
        })
    }

    pub(super) async fn read_mob_spawn<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let entity_type = input.read_i8().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let yaw = input.read_i8().await?;
        let pitch = input.read_i8().await?;
        let data_stream = Metadata::read_from(input).await?;

        Ok(PacketData::MobSpawn {
            entity_id,
            entity_type,
            x,
            y,
            z,
            yaw,
            pitch,
            data_stream,
        })
    }

    pub(super) async fn read_add_painting<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let title = read_ucs2(input).await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let direction = input.read_i32().await?;

        Ok(PacketData::AddPainting {
            entity_id,
            title,
            x,
            y,
            z,
            direction,
        })
    }

    pub(super) async fn read_stance_update<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let unknown_float1 = f32::from_bits(input.read_u32().await?);
        let unknown_float2 = f32::from_bits(input.read_u32().await?);
        let unknown_float3 = f32::from_bits(input.read_u32().await?);
        let unknown_float4 = f32::from_bits(input.read_u32().await?);
        let unknown_bool1 = input.read_i8().await? == 0;
        let unknown_bool2 = input.read_i8().await? == 0;

        Ok(PacketData::StanceUpdate {
            unknown_float1,
            unknown_float2,
            unknown_float3,
            unknown_float4,
            unknown_bool1,
            unknown_bool2,
        })
    }

    

    pub(super) async fn read_generic<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        Ok(PacketData::DisconnectOrKick {
            reason: "Invalid packet received".into(),
        })
    }
}
