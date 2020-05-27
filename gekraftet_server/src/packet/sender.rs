use super::utils::*;
use super::{Metadata, PacketData, PacketId};
use tokio::io::{AsyncWriteExt, Result as IoResult};

impl PacketData {
    pub(super) async fn write_keep_alive<I>(input: &mut I) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::KeepAlive.packet_id()).await?;
        Ok(())
    }

    pub(super) async fn write_login_request<I>(
        input: &mut I,
        id: i32,
        username: Box<str>,
        seed: u64,
        dimension: u8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::LoginRequest.packet_id()).await?;
        input.write_i32(id).await?;
        write_ucs2(input, username).await?;
        input.write_u64(seed).await?;
        input.write_u8(dimension).await?;
        Ok(())
    }

    pub(super) async fn write_handshake<I>(
        input: &mut I,
        username_or_hash: Box<str>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::Handshake.packet_id()).await?;
        write_ucs2(input, username_or_hash).await?;
        Ok(())
    }

    pub(super) async fn write_chat_message<I>(input: &mut I, message: Box<str>) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::ChatMessage.packet_id()).await?;
        write_ucs2(input, message).await?;
        Ok(())
    }

    pub(super) async fn write_time_update<I>(input: &mut I, ticks: i64) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::TimeUpdate.packet_id()).await?;
        input.write_i64(ticks).await?;
        Ok(())
    }

    pub(super) async fn write_entity_equipment<I>(
        input: &mut I,
        entity_id: i32,
        slot: i16,
        item_id: i16,
        unknown: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::EntityEquipment.packet_id())
            .await?;
        input.write_i32(entity_id).await?;
        input.write_i16(slot).await?;
        input.write_i16(item_id).await?;
        input.write_i16(unknown).await?;
        Ok(())
    }

    pub(super) async fn write_spawn_position<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::SpawnPosition.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        Ok(())
    }

    pub(super) async fn write_use_entity<I>(
        input: &mut I,
        source_entity: i32,
        target_entity: i32,
        left_click: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::UseEntity.packet_id()).await?;
        input.write_i32(source_entity).await?;
        input.write_i32(target_entity).await?;
        input.write_u8(left_click as u8).await?;
        Ok(())
    }

    pub(super) async fn write_update_health<I>(input: &mut I, health: i16) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::UpdateHealth.packet_id()).await?;
        input.write_i16(health).await?;
        Ok(())
    }

    pub(super) async fn write_respawn<I>(input: &mut I, dimension: i8) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::Respawn.packet_id()).await?;
        input.write_i8(dimension).await?;
        Ok(())
    }

    pub(super) async fn write_player_flying<I>(input: &mut I, on_ground: bool) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PlayerFlying.packet_id()).await?;
        input.write_u8(on_ground as u8).await?;
        Ok(())
    }

    pub(super) async fn write_player_position<I>(
        input: &mut I,
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        on_ground: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PlayerPosition.packet_id()).await?;
        input.write_u64(x.to_bits()).await?;
        input.write_u64(y.to_bits()).await?;
        input.write_u64(stance.to_bits()).await?;
        input.write_u64(z.to_bits()).await?;
        input.write_u8(on_ground as u8).await?;
        Ok(())
    }

    pub(super) async fn write_player_look<I>(
        input: &mut I,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PlayerLook.packet_id()).await?;
        input.write_u32(yaw.to_bits()).await?;
        input.write_u32(pitch.to_bits()).await?;
        input.write_u8(on_ground as u8).await?;
        Ok(())
    }

    pub(super) async fn write_player_position_and_look<I>(
        input: &mut I,
        x: f64,
        y: f64,
        stance: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::PlayerPositionAndLook.packet_id())
            .await?;
        input.write_u64(x.to_bits()).await?;
        input.write_u64(y.to_bits()).await?;
        input.write_u64(stance.to_bits()).await?;
        input.write_u64(z.to_bits()).await?;
        input.write_u32(yaw.to_bits()).await?;
        input.write_u32(pitch.to_bits()).await?;
        input.write_u8(on_ground as u8).await?;
        Ok(())
    }

    pub(super) async fn write_player_digging<I>(
        input: &mut I,
        status: u8,
        x: i32,
        y: i32,
        z: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PlayerDigging.packet_id()).await?;
        input.write_u8(status).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        Ok(())
    }

    pub(super) async fn write_player_block_placement<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
        direction: i8,
        block_id: i16,
        amount: i8,
        damage: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::PlayerBlockPlacement.packet_id())
            .await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i8(direction).await?;
        input.write_i16(block_id).await?;
        input.write_i8(amount).await?;
        input.write_i16(damage).await?;
        Ok(())
    }

    pub(super) async fn write_holding_change<I>(input: &mut I, slot_id: u16) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::HoldingChange.packet_id()).await?;
        input.write_u16(slot_id).await?;
        Ok(())
    }

    pub(super) async fn write_use_bed<I>(
        input: &mut I,
        player_id: i32,
        in_bed: u8,
        x: i32,
        y: i32,
        z: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::UseBed.packet_id()).await?;
        input.write_i32(player_id).await?;
        input.write_u8(in_bed).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        Ok(())
    }

    pub(super) async fn write_use_animation<I>(
        input: &mut I,
        player_id: i32,
        animation: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::UseAnimation.packet_id()).await?;
        input.write_i32(player_id).await?;
        input.write_i8(animation).await?;
        Ok(())
    }

    pub(super) async fn write_entity_action<I>(
        input: &mut I,
        entity_id: i32,
        action: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityAction.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i8(action).await?;
        Ok(())
    }

    pub(super) async fn write_named_entity_spawn<I>(
        input: &mut I,
        entity_id: i32,
        name: Box<str>,
        x: i32,
        y: i32,
        z: i32,
        rotation: u8,
        pitch: u8,
        current_item: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::NamedEntitySpawn.packet_id())
            .await?;
        input.write_i32(entity_id).await?;
        write_ucs2(input, name).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_u8(rotation).await?;
        input.write_u8(pitch).await?;
        input.write_i16(current_item).await?;
        Ok(())
    }

    pub(super) async fn write_pickup_spawn<I>(
        input: &mut I,
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
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PickupSpawn.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i16(item).await?;
        input.write_i8(count).await?;
        input.write_i16(data).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_u8(rotation).await?;
        input.write_u8(pitch).await?;
        input.write_u8(roll).await?;
        Ok(())
    }

    pub(super) async fn write_collect_item<I>(
        input: &mut I,
        collected_eid: i32,
        collector_eid: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::CollectItem.packet_id()).await?;
        input.write_i32(collected_eid).await?;
        input.write_i32(collector_eid).await?;
        Ok(())
    }

    pub(super) async fn write_add_object<I>(
        input: &mut I,
        entity_id: i32,
        object_type: i8,
        x: i32,
        y: i32,
        z: i32,
        unknown_flag: i32,
        unknown_short1: i16,
        unknown_short2: i16,
        unknown_short3: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::AddObject.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i8(object_type).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i32(unknown_flag).await?;
        input.write_i16(unknown_short1).await?;
        input.write_i16(unknown_short2).await?;
        input.write_i16(unknown_short3).await?;
        Ok(())
    }

    pub(super) async fn write_mob_spawn<I>(
        input: &mut I,
        entity_id: i32,
        entity_type: i8,
        x: i32,
        y: i32,
        z: i32,
        yaw: i8,
        pitch: i8,
        data_stream: Metadata,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::MobSpawn.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i8(entity_type).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i8(yaw).await?;
        input.write_i8(pitch).await?;
        data_stream.write_to(input).await?;
        Ok(())
    }

    pub(super) async fn write_add_painting<I>(
        input: &mut I,
        entity_id: i32,
        title: Box<str>,
        x: i32,
        y: i32,
        z: i32,
        direction: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::AddPainting.packet_id()).await?;
        input.write_i32(entity_id).await?;
        write_ucs2(input, title).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i32(direction).await?;
        Ok(())
    }

    pub(super) async fn write_stance_update<I>(
        input: &mut I,
        unknown_float1: f32,
        unknown_float2: f32,
        unknown_float3: f32,
        unknown_float4: f32,
        unknown_bool1: bool,
        unknown_bool2: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::StanceUpdate.packet_id()).await?;
        input.write_u32(unknown_float1.to_bits()).await?;
        input.write_u32(unknown_float2.to_bits()).await?;
        input.write_u32(unknown_float3.to_bits()).await?;
        input.write_u32(unknown_float4.to_bits()).await?;
        input.write_u8(unknown_bool1 as u8).await?;
        input.write_u8(unknown_bool2 as u8).await?;
        Ok(())
    }

    pub(super) async fn write_entity_velocity<I>(
        input: &mut I,
        entity_id: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityVelocity.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i16(velocity_x).await?;
        input.write_i16(velocity_y).await?;
        input.write_i16(velocity_z).await?;
        Ok(())
    }

    pub(super) async fn write_destroy_entity<I>(input: &mut I, entity_id: i32) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::DestroyEntity.packet_id()).await?;
        input.write_i32(entity_id).await?;
        Ok(())
    }

    pub(super) async fn write_entity_unchanged<I>(input: &mut I, entity_id: i32) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::EntityUnchanged.packet_id())
            .await?;
        input.write_i32(entity_id).await?;
        Ok(())
    }

    pub(super) async fn write_entity_relative_move<I>(
        input: &mut I,
        entity_id: i32,
        dx: i8,
        dy: i8,
        dz: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::EntityRelativeMove.packet_id())
            .await?;
        input.write_i32(entity_id).await?;
        input.write_i8(dx).await?;
        input.write_i8(dy).await?;
        input.write_i8(dz).await?;
        Ok(())
    }

    pub(super) async fn write_entity_look<I>(
        input: &mut I,
        entity_id: i32,
        yaw: i8,
        pitch: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityLook.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i8(yaw).await?;
        input.write_i8(pitch).await?;
        Ok(())
    }

    pub(super) async fn write_entity_look_and_relative_move<I>(
        input: &mut I,
        entity_id: i32,
        dx: i8,
        dy: i8,
        dz: i8,
        yaw: i8,
        pitch: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::EntityLookAndRelativeMove.packet_id())
            .await?;
        input.write_i32(entity_id).await?;
        input.write_i8(dx).await?;
        input.write_i8(dy).await?;
        input.write_i8(dz).await?;
        input.write_i8(yaw).await?;
        input.write_i8(pitch).await?;
        Ok(())
    }

    pub(super) async fn write_entity_teleport<I>(
        input: &mut I,
        entity_id: i32,
        dx: i32,
        dy: i32,
        dz: i32,
        yaw: i8,
        pitch: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityTeleport.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i32(dx).await?;
        input.write_i32(dy).await?;
        input.write_i32(dz).await?;
        input.write_i8(yaw).await?;
        input.write_i8(pitch).await?;
        Ok(())
    }

    pub(super) async fn write_entity_status<I>(
        input: &mut I,
        entity_id: i32,
        status: u8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityStatus.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_u8(status).await?;
        Ok(())
    }

    pub(super) async fn write_attach_entity<I>(
        input: &mut I,
        entity_id: i32,
        vehicle_id: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::AttachEntity.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_i32(vehicle_id).await?;
        Ok(())
    }

    pub(super) async fn write_entity_metadata<I>(
        input: &mut I,
        entity_id: i32,
        metadata: Metadata,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::EntityMetadata.packet_id()).await?;
        input.write_i32(entity_id).await?;
        metadata.write_to(input).await?;
        Ok(())
    }

    pub(super) async fn write_pre_chunk<I>(
        input: &mut I,
        x: i32,
        z: i32,
        init_chunk: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::PreChunk.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(z).await?;
        input.write_u8(init_chunk as u8).await?;
        Ok(())
    }

    pub(super) async fn write_map_chunk<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
        size_x: i8,
        size_y: i8,
        size_z: i8,
        compressed_size: i32,
        compressed_data: Box<[u8]>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::MapChunk.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i8(size_x).await?;
        input.write_i8(size_y).await?;
        input.write_i8(size_z).await?;
        input.write_i32(compressed_size).await?;
        input.write_all(compressed_data.as_ref()).await?;
        Ok(())
    }

    pub(super) async fn write_multi_block_change<I>(
        input: &mut I,
        x: i32,
        z: i32,
        array_size: i16,
        coordinate_array: Box<[u16]>,
        type_array: Box<[u16]>,
        metadata_array: Box<[u16]>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::MultiBlockChange.packet_id())
            .await?;
        input.write_i32(x).await?;
        input.write_i32(z).await?;
        input.write_i16(array_size).await?;
        for i in 0..array_size as usize {
            input.write_u16(coordinate_array[i]).await?
        }
        for i in 0..array_size as usize {
            input.write_u16(type_array[i]).await?
        }
        for i in 0..array_size as usize {
            input.write_u16(metadata_array[i]).await?
        }
        Ok(())
    }

    pub(super) async fn write_block_change<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
        block_type: i8,
        block_metadata: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::BlockChange.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i8(block_type).await?;
        input.write_i8(block_metadata).await?;
        Ok(())
    }

    pub(super) async fn write_block_action<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
        states: [u8; 2],
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::BlockAction.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_u8(states[0]).await?;
        input.write_u8(states[1]).await?;
        Ok(())
    }

    pub(super) async fn write_explosion<I>(
        input: &mut I,
        x: f64,
        y: f64,
        z: f64,
        radius: f32,
        record_count: i32,
        record: Box<[[u8; 3]]>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::Explosion.packet_id()).await?;
        input.write_u64(x.to_bits()).await?;
        input.write_u64(y.to_bits()).await?;
        input.write_u64(z.to_bits()).await?;
        input.write_u32(radius.to_bits()).await?;
        input.write_i32(record_count).await?;
        for x in 0..record_count as usize {
            input.write_all(&record[x]).await?;
        }
        Ok(())
    }

    pub(super) async fn write_sound_effect<I>(
        input: &mut I,
        effect_id: i32,
        x: i32,
        y: i32,
        z: i32,
        sound_data: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::SoundEffect.packet_id()).await?;
        input.write_i32(effect_id).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        input.write_i32(sound_data).await?;
        Ok(())
    }

    pub(super) async fn write_new_state<I>(input: &mut I, reason_code: u8) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::NewState.packet_id()).await?;
        input.write_u8(reason_code).await?;
        Ok(())
    }

    pub(super) async fn write_thunderbolt<I>(
        input: &mut I,
        entity_id: i32,
        unknown: bool,
        x: i32,
        y: i32,
        z: i32,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::Thunderbolt.packet_id()).await?;
        input.write_i32(entity_id).await?;
        input.write_u8(unknown as u8).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        Ok(())
    }

    pub(super) async fn write_open_window<I>(
        input: &mut I,
        window_id: i8,
        inventory_type: i8,
        window_title: Box<str>,
        slots_number: u8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::OpenWindow.packet_id()).await?;
        input.write_i8(window_id).await?;
        input.write_i8(inventory_type).await?;
        write_utf8(input, window_title).await?;
        input.write_u8(slots_number).await?;
        Ok(())
    }

    pub(super) async fn write_close_window<I>(input: &mut I, window_id: i8) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::CloseWindow.packet_id()).await?;
        input.write_i8(window_id).await?;
        Ok(())
    }

    pub(super) async fn write_window_click<I>(
        input: &mut I,
        window_id: i8,
        clicked_slot: i16,
        right_clicked: bool,
        action_number: i16,
        shift_clicked: bool,
        item_id: i16,
        item_count: i8,
        item_uses: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::WindowClick.packet_id()).await?;
        input.write_i8(window_id).await?;
        input.write_i16(clicked_slot).await?;
        input.write_u8(right_clicked as u8).await?;
        input.write_i16(action_number).await?;
        input.write_u8(shift_clicked as u8).await?;
        input.write_i16(item_id).await?;
        input.write_i8(item_count).await?;
        input.write_i16(item_uses).await?;
        Ok(())
    }

    pub(super) async fn write_set_slot<I>(
        input: &mut I,
        window_id: i8,
        update_slot: i16,
        item_id: i16,
        item_count: i8,
        item_uses: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::SetSlot.packet_id()).await?;
        input.write_i8(window_id).await?;
        input.write_i16(update_slot).await?;
        input.write_i16(item_id).await?;
        input.write_i8(item_count).await?;
        input.write_i16(item_uses).await?;
        Ok(())
    }

    pub(super) async fn write_window_items<I>(
        input: &mut I,
        window_id: i8,
        item_count: i16,
        payload: Box<[(i16, Option<(i8, i16)>)]>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::WindowItems.packet_id()).await?;
        input.write_i8(window_id).await?;
        input.write_i16(item_count).await?;
        for (id, optional) in payload.iter() {
            input.write_i16(*id).await?;
            if let Some((count, uses)) = optional {
                input.write_i8(*count).await?;
                input.write_i16(*uses).await?;
            }
        }
        Ok(())
    }

    pub(super) async fn write_update_progress_bar<I>(
        input: &mut I,
        window_id: i8,
        progress_bar: i16,
        value: i16,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::UpdateProgressBar.packet_id())
            .await?;
        input.write_i8(window_id).await?;
        input.write_i16(progress_bar).await?;
        input.write_i16(value).await?;
        Ok(())
    }

    pub(super) async fn write_transaction<I>(
        input: &mut I,
        window_id: i8,
        action_number: i16,
        accepted: bool,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::Transaction.packet_id()).await?;
        input.write_i8(window_id).await?;
        input.write_i16(action_number).await?;
        input.write_u8(accepted as u8).await?;
        Ok(())
    }

    pub(super) async fn write_update_sign<I>(
        input: &mut I,
        x: i32,
        y: i32,
        z: i32,
        text: Box<str>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::UpdateSign.packet_id()).await?;
        input.write_i32(x).await?;
        input.write_i32(y).await?;
        input.write_i32(z).await?;
        for t in text.lines() {
            write_ucs2(input, t).await?;
        }
        Ok(())
    }

    pub(super) async fn write_item_data<I>(
        input: &mut I,
        item_type: i16,
        item_id: i16,
        text_length: u8,
        text: Box<[u8]>,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input.write_u8(PacketId::ItemData.packet_id()).await?;
        input.write_i16(item_type).await?;
        input.write_i16(item_id).await?;
        input.write_u8(text_length).await?;
        input.write_all(text.as_ref()).await?;
        Ok(())
    }

    pub(super) async fn write_increment_statistic<I>(
        input: &mut I,
        statistic_id: i32,
        amount: i8,
    ) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::IncrementStatistic.packet_id())
            .await?;
        input.write_i32(statistic_id).await?;
        input.write_i8(amount).await?;
        Ok(())
    }

    pub(super) async fn write_disconnect_or_kick<I>(input: &mut I, reason: Box<str>) -> IoResult<()>
    where
        I: AsyncWriteExt + Unpin,
    {
        input
            .write_u8(PacketId::DisconnectOrKick.packet_id())
            .await?;
        write_ucs2(input, reason).await?;
        Ok(())
    }
}
