mod data;
mod id;
mod metadata;
mod receiver;
mod sender;
mod utils;

pub use data::*;
pub use metadata::*;
pub use id::*;

use tokio::io::{ AsyncReadExt, Result as IoResult };

pub struct Packet {
    data: PacketData,
}

impl Packet {
    pub fn new(data: PacketData) -> Self {
        Self { data }
    }

    pub async fn read_packet<I>(input: &mut I) -> IoResult<Self> 
        where I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt
    {
        let packet_id = PacketId::from_packet_id(input.read_u8().await?);
        let packet = match packet_id {
            Some(PacketId::KeepAlive) => PacketData::read_keep_alive(input).await?,
            Some(PacketId::LoginRequest) => PacketData::read_login_request(input).await?,
            Some(PacketId::Handshake) => PacketData::read_handshake(input).await?,
            Some(PacketId::ChatMessage) => PacketData::read_chat_message(input).await?,
            Some(PacketId::TimeUpdate) => PacketData::read_time_update(input).await?,
            Some(PacketId::EntityEquipment) => PacketData::read_entity_equipment(input).await?,
            Some(PacketId::SpawnPosition) => PacketData::read_spawn_position(input).await?,
            Some(PacketId::UseEntity) => PacketData::read_use_entity(input).await?,
            Some(PacketId::UpdateHealth) => PacketData::read_update_health(input).await?,
            Some(PacketId::Respawn) => PacketData::read_respawn(input).await?,
            Some(PacketId::PlayerFlying) => PacketData::read_player_flying(input).await?,
            Some(PacketId::PlayerPosition) => PacketData::read_player_position(input).await?,
            Some(PacketId::PlayerLook) => PacketData::read_player_look(input).await?,
            Some(PacketId::PlayerPositionAndLook) => PacketData::read_player_position_and_look(input).await?,
            Some(PacketId::PlayerDigging) => PacketData::read_player_digging(input).await?,
            Some(PacketId::PlayerBlockPlacement) => PacketData::read_player_block_placement(input).await?,
            Some(PacketId::HoldingChange) => PacketData::read_holding_change(input).await?,
            Some(PacketId::UseBed) => PacketData::read_use_bed(input).await?,
            Some(PacketId::UseAnimation) => PacketData::read_use_animation(input).await?,
            Some(PacketId::EntityAction) => PacketData::read_entity_action(input).await?,
            Some(PacketId::NamedEntitySpawn) => PacketData::read_named_entity_spawn(input).await?,
            Some(PacketId::PickupSpawn) => PacketData::read_pickup_spawn(input).await?,
            Some(PacketId::CollectItem) => PacketData::read_collect_item(input).await?,
            Some(PacketId::AddObject) => PacketData::read_add_object(input).await?,
            Some(PacketId::MobSpawn) => PacketData::read_mob_spawn(input).await?,
            Some(PacketId::AddPainting) => PacketData::read_add_painting(input).await?,
            Some(PacketId::StanceUpdate) => PacketData::read_stance_update(input).await?,
            Some(PacketId::EntityVelocity) => PacketData::read_entity_velocity(input).await?,
            Some(PacketId::DestroyEntity) => PacketData::read_destroy_entity(input).await?,
            Some(PacketId::EntityUnchanged) => PacketData::read_entity_unchanged(input).await?,
            Some(PacketId::EntityRelativeMove) => PacketData::read_entity_relative_move(input).await?,
            Some(PacketId::EntityLook) => PacketData::read_entity_look(input).await?,
            Some(PacketId::EntityLookAndRelativeMove) => PacketData::read_entity_look_and_relative_move(input).await?,
            Some(PacketId::EntityTeleport) => PacketData::read_entity_teleport(input).await?,
            Some(PacketId::EntityStatus) => PacketData::read_entity_status(input).await?,
            Some(PacketId::AttachEntity) => PacketData::read_attach_entity(input).await?,
            Some(PacketId::EntityMetadata) => PacketData::read_entity_metadata(input).await?,
            Some(PacketId::PreChunk) => PacketData::read_pre_chunk(input).await?,
            Some(PacketId::MapChunk) => PacketData::read_map_chunk(input).await?,
            Some(PacketId::MultiBlockChange) => PacketData::read_multi_block_change(input).await?,
            Some(PacketId::BlockChange) => PacketData::read_block_change(input).await?,
            Some(PacketId::BlockAction) => PacketData::read_block_action(input).await?,
            Some(PacketId::Explosion) => PacketData::read_explosion(input).await?,
            Some(PacketId::SoundEffect) => PacketData::read_sound_effect(input).await?,
            Some(PacketId::NewState) => PacketData::read_new_state(input).await?,
            Some(PacketId::Thunderbolt) => PacketData::read_thunderbolt(input).await?,
            Some(PacketId::OpenWindow) => PacketData::read_open_window(input).await?,
            Some(PacketId::CloseWindow) => PacketData::read_close_window(input).await?,
            Some(PacketId::WindowClick) => PacketData::read_window_click(input).await?,
            Some(PacketId::SetSlot) => PacketData::read_set_slot(input).await?,
            Some(PacketId::WindowItems) => PacketData::read_window_items(input).await?,
            Some(PacketId::UpdateProgressBar) => PacketData::read_update_progress_bar(input).await?,
            Some(PacketId::Transaction) => PacketData::read_transaction(input).await?,
            Some(PacketId::UpdateSign) => PacketData::read_update_sign(input).await?,
            Some(PacketId::ItemData) => PacketData::read_item_data(input).await?,
            Some(PacketId::IncrementStatistic) => PacketData::read_increment_statistic(input).await?,
            Some(PacketId::DisconnectOrKick) => PacketData::read_disconnect_or_kick(input).await?,

            _ => PacketData::read_generic(input).await?,
        };

        println!("{:?}", packet);

        Ok(Self { data: packet })
    }

    pub async fn write_packet<I>(&self, input: &mut I) -> IoResult<()> 
        where I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt
    {
        match self.data.clone() {
            PacketData::KeepAlive => PacketData::write_keep_alive(input).await?,

            PacketData::LoginRequest {
                id,
                username,
                seed,
                dimension,
            } => PacketData::write_login_request(input, id, username, seed, dimension).await?,

            PacketData::Handshake {
                username_or_hash,
            } => PacketData::write_handshake(input, username_or_hash).await?,

            PacketData::ChatMessage {
                message,
            } => PacketData::write_chat_message(input, message).await?,

            PacketData::TimeUpdate {
                ticks,
            } => PacketData::write_time_update(input, ticks).await?,

            PacketData::EntityEquipment {
                entity_id,
                slot,
                item_id,
                unknown,
            } => PacketData::write_entity_equipment(input, entity_id, slot, item_id, unknown).await?,

            PacketData::SpawnPosition {
                x,
                y,
                z,
            } => PacketData::write_spawn_position(input, x, y, z).await?,

            PacketData::UseEntity {
                source_entity,
                target_entity,
                left_click,
            } => PacketData::write_use_entity(input, source_entity, target_entity, left_click).await?,

            PacketData::UpdateHealth {
                health,
            } => PacketData::write_update_health(input, health).await?,

            PacketData::Respawn {
                dimension,
            } => PacketData::write_respawn(input, dimension).await?,

            PacketData::PlayerFlying {
                on_ground,
            } => PacketData::write_player_flying(input, on_ground).await?,

            PacketData::PlayerPosition {
                x,
                y,
                stance,
                z,
                on_ground,
            } => PacketData::write_player_position(input, x, y, stance, z, on_ground).await?,

            PacketData::PlayerLook {
                yaw,
                pitch,
                on_ground,
            } => PacketData::write_player_look(input, yaw, pitch, on_ground).await?,

            PacketData::PlayerPositionAndLook {
                x,
                y,
                stance,
                z,
                yaw,
                pitch,
                on_ground,
            } => PacketData::write_player_position_and_look(input, x, y, stance, z, yaw, pitch, on_ground).await?,

            PacketData::PlayerDigging {
                status, // TODO: use an actual enum for status
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
            } => PacketData::write_player_digging(input, status, x, y, z).await?,

            PacketData::PlayerBlockPlacement {
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
                direction,
                block_id,
                amount,
                damage,
            } => PacketData::write_player_block_placement(input, x, y, z, direction, block_id, amount, damage).await?,

            PacketData::HoldingChange {
                slot_id,
            } => PacketData::write_holding_change(input, slot_id).await?,

            PacketData::UseBed {
                player_id,
                in_bed,
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
            } => PacketData::write_use_bed(input, player_id, in_bed, x, y, z).await?,

            PacketData::UseAnimation {
                player_id,
                animation, // TODO: actual enum
            } => PacketData::write_use_animation(input, player_id, animation).await?,

            PacketData::EntityAction {
                entity_id,
                action, // TODO: actual enum
            } => PacketData::write_entity_action(input, entity_id, action).await?,

            PacketData::NamedEntitySpawn {
                entity_id,
                name,
                x,
                y,
                z,
                rotation,
                pitch,
                current_item,
            } => PacketData::write_named_entity_spawn(input, entity_id, name, x, y, z, rotation, pitch, current_item).await?,

            PacketData::PickupSpawn {
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
            } => PacketData::write_pickup_spawn(input, entity_id, item, count, data, x, y, z, rotation, pitch, roll).await?,

            PacketData::CollectItem {
                collected_eid,
                collector_eid,
            } => PacketData::write_collect_item(input, collected_eid, collector_eid).await?,

            PacketData::AddObject {
                entity_id,
                object_type, // TODO: actual enum
                x,
                y,
                z,
                unknown_flag,
                unknown_short1,
                unknown_short2,
                unknown_short3,
            } => PacketData::write_add_object(input, entity_id, object_type, x, y, z, unknown_flag, unknown_short1, unknown_short2, unknown_short3).await?,

            PacketData::MobSpawn {
                entity_id,
                entity_type, // TODO: actual enum
                x,
                y,
                z,
                yaw,
                pitch,
                data_stream,
            } => PacketData::write_mob_spawn(input, entity_id, entity_type, x, y, z, yaw, pitch, data_stream).await?,

            PacketData::AddPainting {
                entity_id,
                title,
                x,
                y,
                z,
                direction,
            } => PacketData::write_add_painting(input, entity_id, title, x, y, z, direction).await?,

            PacketData::StanceUpdate {
                // Unused?
                unknown_float1,
                unknown_float2,
                unknown_float3,
                unknown_float4,
                unknown_bool1,
                unknown_bool2,
            } => PacketData::write_stance_update(input, unknown_float1, unknown_float2, unknown_float3, unknown_float4, unknown_bool1, unknown_bool2).await?,

            PacketData::EntityVelocity {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            } => PacketData::write_entity_velocity(input, entity_id, velocity_x, velocity_y, velocity_z).await?,

            PacketData::DestroyEntity {
                entity_id,
            } => PacketData::write_destroy_entity(input, entity_id).await?,

            PacketData::EntityUnchanged {
                entity_id,
            } => PacketData::write_entity_unchanged(input, entity_id).await?,

            PacketData::EntityRelativeMove {
                entity_id,
                dx,
                dy,
                dz,
            } => PacketData::write_entity_relative_move(input, entity_id, dx, dy, dz).await?,

            PacketData::EntityLook {
                entity_id,
                yaw,
                pitch,
            } => PacketData::write_entity_look(input, entity_id, yaw, pitch).await?,

            PacketData::EntityLookAndRelativeMove {
                entity_id,
                dx,
                dy,
                dz,
                yaw,
                pitch,
            } => PacketData::write_entity_look_and_relative_move(input, entity_id, dx, dy, dz, yaw, pitch).await?,

            PacketData::EntityTeleport {
                entity_id,
                dx,
                dy,
                dz,
                yaw,
                pitch,
            } => PacketData::write_entity_teleport(input, entity_id, dx, dy, dz, yaw, pitch).await?,

            PacketData::EntityStatus {
                entity_id,
                status, // TODO: actual enum
            } => PacketData::write_entity_status(input, entity_id, status).await?,

            PacketData::AttachEntity {
                entity_id,
                vehicle_id,
            } => PacketData::write_attach_entity(input, entity_id, vehicle_id).await?,

            PacketData::EntityMetadata {
                entity_id,
                metadata,
            } => PacketData::write_entity_metadata(input, entity_id, metadata).await?,

            PacketData::PreChunk {
                x,
                z,
                init_chunk,
            } => PacketData::write_pre_chunk(input, x, z, init_chunk).await?,

            PacketData::MapChunk {
                x,
                y, // NOTE: the vanilla game uses i16 here
                z,
                size_x,
                size_y,
                size_z,
                compressed_size,
                compressed_data,
            } => PacketData::write_map_chunk(input, x, y, z, size_x, size_y, size_z, compressed_size, compressed_data).await?,

            PacketData::MultiBlockChange {
                x,
                z,
                array_size,
                coordinate_array,
                type_array,
                metadata_array,
            } => PacketData::write_multi_block_change(input, x, z, array_size, coordinate_array, type_array, metadata_array).await?,

            PacketData::BlockChange {
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
                block_type,
                block_metadata,
            } => PacketData::write_block_change(input, x, y, z, block_type, block_metadata).await?,

            PacketData::BlockAction {
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
                states,
            } => PacketData::write_block_action(input, x, y, z, states).await?,

            PacketData::Explosion {
                x,
                y,
                z,
                radius,
                record_count,
                record,
            } => PacketData::write_explosion(input, x, y, z, radius, record_count, record).await?,

            PacketData::SoundEffect {
                effect_id,
                x,
                y, // NOTE: the vanilla game uses i8 here
                z,
                sound_data,
            } => PacketData::write_sound_effect(input, effect_id, x, y, z, sound_data).await?,

            PacketData::NewState {
                reason_code,
            } => PacketData::write_new_state(input, reason_code).await?,

            PacketData::Thunderbolt {
                entity_id,
                unknown,
                x,
                y,
                z,
            } => PacketData::write_thunderbolt(input, entity_id, unknown, x, y, z).await?,

            PacketData::OpenWindow {
                window_id,
                inventory_type,
                window_title, // NOTE: this is the only UTF8 string here
                slots_number,
            } => PacketData::write_open_window(input, window_id, inventory_type, window_title, slots_number).await?,

            PacketData::CloseWindow {
                window_id,
            } => PacketData::write_close_window(input, window_id).await?,

            PacketData::WindowClick {
                window_id,
                clicked_slot,
                right_clicked,
                action_number,
                shift_clicked,
                item_id,
                item_count,
                item_uses,
            } => PacketData::write_window_click(input, window_id, clicked_slot, right_clicked, action_number, shift_clicked, item_id, item_count, item_uses).await?,

            PacketData::SetSlot {
                window_id,
                update_slot,
                item_id,
                item_count,
                item_uses,
            } => PacketData::write_set_slot(input, window_id, update_slot, item_id, item_count, item_uses).await?,

            PacketData::WindowItems {
                window_id,
                item_count,
                // stores (item ID, Some(count, uses)) if item ID != -1
                payload,
            } => PacketData::write_window_items(input, window_id, item_count, payload).await?,

            PacketData::UpdateProgressBar {
                window_id,
                progress_bar, // TODO: use an actual enum (type depends on block?)
                value,
            } => PacketData::write_update_progress_bar(input, window_id, progress_bar, value).await?,

            PacketData::Transaction {
                window_id,
                action_number,
                accepted,
            } => PacketData::write_transaction(input, window_id, action_number, accepted).await?,

            PacketData::UpdateSign {
                x,
                y, // NOTE: the vanilla game uses i16 here
                z,
                text, // NOTE, and each line starts with its length
            } => PacketData::write_update_sign(input, x, y, z, text).await?,

            PacketData::ItemData {
                item_type,
                item_id, // should be called damage value
                text_length,
                text,
            } => PacketData::write_item_data(input, item_type, item_id, text_length, text).await?,

            PacketData::IncrementStatistic {
                statistic_id,
                amount,
            } => PacketData::write_increment_statistic(input, statistic_id, amount).await?,

            PacketData::DisconnectOrKick {
                reason,
            } => PacketData::write_disconnect_or_kick(input, reason).await?,
        };

        Ok(())
    }
}
