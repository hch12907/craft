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
        let y = input.read_i8().await? as i32;
        let z = input.read_i32().await?;

        Ok(PacketData::PlayerDigging { status, x, y, z })
    }

    pub(super) async fn read_player_block_placement<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i8().await? as i32;
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
        let y = input.read_i8().await? as i32;
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

    pub(super) async fn read_entity_velocity<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let velocity_x = input.read_i16().await?;
        let velocity_y = input.read_i16().await?;
        let velocity_z = input.read_i16().await?;

        Ok(PacketData::EntityVelocity {
            entity_id,
            velocity_x,
            velocity_y,
            velocity_z,
        })
    }

    pub(super) async fn read_destroy_entity<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;

        Ok(PacketData::DestroyEntity {
            entity_id,
        })
    }

    pub(super) async fn read_entity_unchanged<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;

        Ok(PacketData::EntityUnchanged {
            entity_id,
        })
    }

    pub(super) async fn read_entity_relative_move<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let dx = input.read_i8().await?;
        let dy = input.read_i8().await?;
        let dz = input.read_i8().await?;

        Ok(PacketData::EntityRelativeMove {
            entity_id,
            dx,
            dy,
            dz,
        })
    }

    pub(super) async fn read_entity_look<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let yaw = input.read_i8().await?;
        let pitch = input.read_i8().await?;

        Ok(PacketData::EntityLook {
            entity_id,
            yaw,
            pitch,
        })
    }

    // woah this is such a long name!
    pub(super) async fn read_entity_look_and_relative_move<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let dx = input.read_i8().await?;
        let dy = input.read_i8().await?;
        let dz = input.read_i8().await?;
        let yaw = input.read_i8().await?;
        let pitch = input.read_i8().await?;

        Ok(PacketData::EntityLookAndRelativeMove {
            entity_id,
            dx,
            dy,
            dz,
            yaw,
            pitch,
        })
    }

    pub(super) async fn read_entity_teleport<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let dx = input.read_i32().await?;
        let dy = input.read_i32().await?;
        let dz = input.read_i32().await?;
        let yaw = input.read_i8().await?;
        let pitch = input.read_i8().await?;

        Ok(PacketData::EntityTeleport {
            entity_id,
            dx,
            dy,
            dz,
            yaw,
            pitch,
        })
    }

    pub(super) async fn read_entity_status<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let status = input.read_u8().await?;

        Ok(PacketData::EntityStatus {
            entity_id,
            status,
        })
    }

    pub(super) async fn read_attach_entity<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let vehicle_id = input.read_u8().await?;

        Ok(PacketData::AttachEntity {
            entity_id,
            vehicle_id,
        })
    }

    pub(super) async fn read_entity_metadata<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let entity_id = input.read_i32().await?;
        let metadata = Metadata::read_from(input).await?;

        Ok(PacketData::EntityMetadata {
            entity_id,
            metadata,
        })
    }

    pub(super) async fn read_pre_chunk<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let z = input.read_i32().await?;
        let init_chunk = input.read_i8().await? == 1;

        Ok(PacketData::PreChunk {
            x,
            z,
            init_chunk,
        })
    }

    pub(super) async fn read_map_chunk<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i16().await? as i32;
        let z = input.read_i32().await?;
        let size_x = input.read_i8().await?;
        let size_y = input.read_i8().await?;
        let size_z = input.read_i8().await?;
        let compressed_size = input.read_i32().await?;
        
        let compressed_data = {
            let mut data = Vec::with_capacity(compressed_size as usize);
            input.take(compressed_size as u64).read_to_end(&mut data).await?;
            data.into_boxed_slice()
        };


        Ok(PacketData::MapChunk {
            x,
            y,
            z,
            size_x,
            size_y,
            size_z,
            compressed_size,
            compressed_data,
        })
    }

    pub(super) async fn read_multi_block_change<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let z = input.read_i32().await?;
        let array_size = input.read_i16().await?;
        
        let coordinate_array = {
            let mut data = Vec::with_capacity(array_size as usize);
            for _ in 0..array_size {
                data.push(input.read_u16().await?);
            }
            data.into_boxed_slice()
        };

        let type_array = {
            let mut data = Vec::with_capacity(array_size as usize);
            for _ in 0..array_size {
                data.push(input.read_u16().await?);
            }
            data.into_boxed_slice()
        };

        let metadata_array = {
            let mut data = Vec::with_capacity(array_size as usize);
            for _ in 0..array_size {
                data.push(input.read_u16().await?);
            }
            data.into_boxed_slice()
        };

        Ok(PacketData::MultiBlockChange {
            x,
            z,
            array_size,
            coordinate_array,
            type_array,
            metadata_array,
        })
    }

    pub(super) async fn read_block_change<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i8().await? as i32;
        let z = input.read_i32().await?;
        let block_type = input.read_i8().await?;
        let block_metadata = input.read_i8().await?;
        
        Ok(PacketData::BlockChange {
            x,
            y,
            z,
            block_type,
            block_metadata,
        })
    }

    pub(super) async fn read_block_action<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let x = input.read_i32().await?;
        let y = input.read_i8().await? as i32;
        let z = input.read_i32().await?;
        let states = [
            input.read_u8().await?,
            input.read_u8().await?,
        ];
        
        Ok(PacketData::BlockAction {
            x,
            y,
            z,
            states
        })
    }

    pub(super) async fn read_explosion<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let x = f64::from_bits(input.read_u64().await?);
        let y = f64::from_bits(input.read_u64().await?);
        let z = f64::from_bits(input.read_u64().await?);
        let radius = f32::from_bits(input.read_u32().await?);
        let record_count = input.read_i32().await?;
        
        let record = {
            let mut data = Vec::with_capacity(record_count as usize);
            for _ in 0..record_count {
                let record = [
                    input.read_u8().await?,
                    input.read_u8().await?,
                    input.read_u8().await?,
                ];
                data.push(record);
            }
            data.into_boxed_slice()
        };
        
        Ok(PacketData::Explosion {
            x,
            y,
            z,
            radius,
            record_count,
            record,
        })
    }

    pub(super) async fn read_sound_effect<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let effect_id = input.read_i32().await?;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;
        let sound_data = input.read_i32().await?;
        
        Ok(PacketData::SoundEffect {
            effect_id,
            x,
            y,
            z,
            sound_data,
        })
    }

    pub(super) async fn read_new_state<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let reason_code = input.read_u8().await?;
        
        Ok(PacketData::NewState {
            reason_code
        })
    }

    pub(super) async fn read_thunderbolt<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let entity_id = input.read_i32().await?;
        let unknown = input.read_i8().await? == 1;
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;

        Ok(PacketData::Thunderbolt {
            entity_id,
            unknown,
            x,
            y,
            z,
        })
    }

    pub(super) async fn read_open_window<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let inventory_type = input.read_i8().await?;
        let window_title = read_utf8(input).await?;
        let slots_number = input.read_u8().await?;

        Ok(PacketData::OpenWindow {
            window_id,
            inventory_type,
            window_title,
            slots_number,
        })
    }

    pub(super) async fn read_close_window<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;

        Ok(PacketData::CloseWindow {
            window_id
        })
    }

    pub(super) async fn read_window_click<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let clicked_slot = input.read_i16().await?;
        let right_clicked = input.read_i8().await? == 1;
        let action_number = input.read_i16().await?;
        let shift_clicked = input.read_i8().await? == 1;
        let item_id = input.read_i16().await?;
        let item_count = input.read_i8().await?;
        let item_uses = input.read_i16().await?;

        Ok(PacketData::WindowClick {
            window_id,
            clicked_slot,
            right_clicked,
            action_number,
            shift_clicked,
            item_id,
            item_count,
            item_uses,
        })
    }

    pub(super) async fn read_set_slot<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let update_slot = input.read_i16().await?;
        let item_id = input.read_i16().await?;
        let item_count = input.read_i8().await?;
        let item_uses = input.read_i16().await?;

        Ok(PacketData::SetSlot {
            window_id,
            update_slot,
            item_id,
            item_count,
            item_uses,
        })
    }

    pub(super) async fn read_window_items<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let item_count = input.read_i16().await?;

        let payload = {
            let mut data = Vec::with_capacity(item_count as usize);
            for _ in 0..item_count {
                let item_id = input.read_i16().await?;
                
                let optional_desc = if item_id == -1 {
                    None
                } else {
                    let item_count = input.read_i8().await?;
                    let item_uses = input.read_i16().await?;
                    Some((item_count, item_uses))
                };

                data.push((item_id, optional_desc))
            }
            data.into_boxed_slice()
        };

        Ok(PacketData::WindowItems {
            window_id,
            item_count,
            payload,
        })
    }

    pub(super) async fn read_update_progress_bar<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let progress_bar = input.read_i16().await?;
        let value = input.read_i16().await?;

        Ok(PacketData::UpdateProgressBar {
            window_id,
            progress_bar,
            value,
        })
    }

    pub(super) async fn read_transaction<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let window_id = input.read_i8().await?;
        let action_number = input.read_i16().await?;
        let accepted = input.read_i8().await? == 1;

        Ok(PacketData::Transaction {
            window_id,
            action_number,
            accepted,
        })
    }

    pub(super) async fn read_update_sign<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let x = input.read_i32().await?;
        let y = input.read_i32().await?;
        let z = input.read_i32().await?;

        let text = (
            read_ucs2(input).await?.to_string()
            + "\n" + &read_ucs2(input).await?
            + "\n" + &read_ucs2(input).await?
            + "\n" + &read_ucs2(input).await?
        ).into_boxed_str();

        Ok(PacketData::UpdateSign {
            x,
            y,
            z,
            text,
        })
    }

    pub(super) async fn read_item_data<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let item_type = input.read_i16().await?;
        let item_id = input.read_i16().await?;
        let text_length = input.read_u8().await?;
        
        let text= {
            let mut t = Vec::with_capacity(text_length as usize);
            input.take(text_length as u64).read_to_end(&mut t).await?;
            t.into_boxed_slice()
        };

        Ok(PacketData::ItemData {
            item_id,
            item_type,
            text_length,
            text,
        })
    }

    pub(super) async fn read_increment_statistic<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin
    {
        let statistic_id = input.read_i32().await?;
        let amount = input.read_i8().await?;

        Ok(PacketData::IncrementStatistic {
            statistic_id,
            amount,
        })
    }

    pub(super) async fn read_disconnect_or_kick<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin,
    {
        let reason = read_ucs2(input).await?;

        Ok(PacketData::DisconnectOrKick {
            reason
        })
    }

    pub(super) async fn read_generic<I>(input: &mut I) -> IoResult<Self>
    where
        I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt,
    {
        const GENERIC_INVALID_PACKET_MESSAGE: &'static str = "gekraftet_server: invalid packet received";
        let reason = GENERIC_INVALID_PACKET_MESSAGE
            .to_string()
            .encode_utf16()
            .collect::<Vec<_>>();

        input.write_i16(GENERIC_INVALID_PACKET_MESSAGE.len() as i16).await?;
        
        for x in reason.iter() {
            input.write_u16(*x).await?;
        }

        Ok(PacketData::DisconnectOrKick {
            reason: GENERIC_INVALID_PACKET_MESSAGE.into()
        })
    }
}
