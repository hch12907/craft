mod data;
mod id;
mod metadata;
mod receiver;
mod utils;

pub use data::*;
pub use metadata::*;
pub use id::*;

use tokio::io::{ AsyncReadExt, Result as IoResult };

pub struct Packet {
    data: PacketData,
}

impl Packet {
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
}
