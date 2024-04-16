use crate::Control;
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::{Debug, Display};

pub mod add_endpoint;
pub mod add_or_update_key_table_entry;
pub mod add_transient_link_key;
pub mod address_table_entry_is_active;
pub mod aes_encrypt;
pub mod aes_mmo_hash;
pub mod binding_is_active;
pub mod bootload_transmit_complete_handler;
pub mod broadcast_network_key_switch;
pub mod broadcast_next_network_key;
pub mod calculate_smacs;
pub mod calculate_smacs283k1;
pub mod calculate_smacs_handler;
pub mod calculate_smacs_handler283k1;
pub mod callback;
pub mod child_id;
pub mod child_join_handler;
pub mod clear_binding_table;
pub mod clear_key_table;
pub mod clear_stored_beacons;
pub mod clear_temporary_data_maybe_store_link_key;
pub mod clear_temporary_data_maybe_store_link_key283k1;
pub mod clear_transient_link_keys;
pub mod counter_rollover_handler;
pub mod custom_frame;
pub mod custom_frame_handler;
pub mod d_gp_send;
pub mod d_gp_sent_handler;
pub mod debug_write;
pub mod delay_test;
pub mod delete_binding;
pub mod dsa_sign;
pub mod dsa_sign_handler;
pub mod dsa_verify;
pub mod dsa_verify283k1;
pub mod dsa_verify_handler;
pub mod duty_cycle_handler;
pub mod echo;
pub mod energy_scan_request;
pub mod energy_scan_result_handler;
pub mod erase_key_table_entry;
pub mod export_key;
pub mod export_link_key_by_eui;
pub mod export_link_key_by_index;
pub mod export_transient_key_by_eui;
pub mod export_transient_key_by_index;
pub mod find_and_rejoin_network;
pub mod find_key_table_entry;
pub mod find_unused_pan_id;
pub mod form_network;
pub mod generate_cbke_keys;
pub mod generate_cbke_keys283k1;
pub mod generate_cbke_keys_handler;
pub mod generate_cbke_keys_handler283k1;
pub mod get_address_table_remote_eui64;
pub mod get_address_table_remote_node_id;
pub mod get_aps_key_info;
pub mod get_beacon_classification_params;
pub mod get_binding;
pub mod get_binding_remote_node_id;
pub mod get_certificate;
pub mod get_certificate283k1;
pub mod get_child_data;
pub mod get_configuration_value;
pub mod get_current_duty_cycle;
pub mod get_current_security_state;
pub mod get_duty_cycle_limits;
pub mod get_duty_cycle_state;
pub mod get_extended_timeout;
pub mod get_extended_value;
pub mod get_first_beacon;
pub mod get_key;
pub mod get_key_table_entry;
pub mod get_library_status;
pub mod get_mfg_token;
pub mod get_multicast_table_entry;
pub mod get_neighbor;
pub mod get_neighbor_frame_counter;
pub mod get_network_key_info;
pub mod get_network_parameters;
pub mod get_next_beacon;
pub mod get_node_id;
pub mod get_num_stored_beacons;
pub mod get_parent_child_parameters;
pub mod get_parent_classification_enabled;
pub mod get_phy_interface_count;
pub mod get_policy;
pub mod get_radio_channel;
pub mod get_radio_parameters;
pub mod get_random_number;
pub mod get_route_table_entry;
pub mod get_routing_shortcut_threshold;
pub mod get_security_key_status;
pub mod get_source_route_table_entry;
pub mod get_source_route_table_filled_size;
pub mod get_source_route_table_total_size;
pub mod get_standalone_bootloader_version_plat_micro_phy;
pub mod get_timer;
pub mod get_token;
pub mod get_token_count;
pub mod get_token_data;
pub mod get_token_info;
pub mod get_transient_key_table_entry;
pub mod get_transient_link_key;
pub mod get_true_random_entropy_source;
pub mod get_value;
pub mod get_xncp_info;
pub mod get_zll_primary_channel_mask;
pub mod get_zll_secondary_channel_mask;
pub mod gp_proxy_table_get_entry;
pub mod gp_proxy_table_lookup;
pub mod gp_proxy_table_process_gp_pairing;
pub mod gp_sink_commission;
pub mod gp_sink_table_clear_all;
pub mod gp_sink_table_find_or_allocate_entry;
pub mod gp_sink_table_get_entry;
pub mod gp_sink_table_init;
pub mod gp_sink_table_lookup;
pub mod gp_sink_table_remove_entry;
pub mod gp_sink_table_set_entry;
pub mod gp_sink_table_set_security_frame_counter;
pub mod gp_translation_table_clear;
pub mod gpep_incoming_message_handler;
pub mod id;
pub mod id_conflict_handler;
pub mod import_key;
pub mod import_link_key;
pub mod import_transient_key;
pub mod incoming_bootload_message_handler;
pub mod incoming_many_to_one_route_request_handler;
pub mod incoming_message_handler;
pub mod incoming_network_status_handler;
pub mod incoming_route_error_handler;
pub mod incoming_sender_eui64_handler;
pub mod invalid_command;
pub mod is_hub_connected;
pub mod is_up_time_long;
pub mod is_zll_network;
pub mod join_network;
pub mod join_network_directly;
pub mod launch_standalone_bootloader;
pub mod leave_network;
pub mod lookup_eui64_by_node_id;
pub mod lookup_node_id_by_eui64;
pub mod mac_filter_match_message_handler;
pub mod mac_passthrough_message_handler;
pub mod maximum_payload_length;
pub mod message_sent_handler;
pub mod mfglib_end;
pub mod mfglib_get_channel;
pub mod mfglib_get_power;
pub mod mfglib_rx_handler;
pub mod mfglib_send_packet;
pub mod mfglib_set_channel;
pub mod mfglib_set_power;
pub mod mfglib_start;
pub mod mfglib_start_stream;
pub mod mfglib_start_tone;
pub mod mfglib_stop_stream;
pub mod mfglib_stop_tone;
pub mod multi_phy_set_radio_channel;
pub mod multi_phy_set_radio_power;
pub mod multi_phy_start;
pub mod multi_phy_stop;
pub mod neighbor_count;
pub mod network_found_handler;
pub mod network_init;
pub mod network_state;
pub mod no_callbacks;
pub mod nop;
pub mod override_current_channel;
pub mod permit_joining;
pub mod poll_complete_handler;
pub mod poll_for_data;
pub mod poll_handler;
pub mod proxy_broadcast;
pub mod raw_transmit_complete_handler;
pub mod read_and_clear_counters;
pub mod read_attribute;
pub mod read_counters;
pub mod remote_delete_binding_handler;
pub mod remote_set_binding_handler;
pub mod remove_device;
pub mod replace_address_table_entry;
pub mod request_link_key;
pub mod reset_node;
pub mod reset_to_factory_defaults;
pub mod save_preinstalled_cbke_data283k1;
pub mod scan_complete_handler;
pub mod send_bootload_message;
pub mod send_broadcast;
pub mod send_link_power_delta_request;
pub mod send_many_to_one_route_request;
pub mod send_multicast;
pub mod send_multicast_with_alias;
pub mod send_pan_id_update;
pub mod send_raw_message;
pub mod send_raw_message_extended;
pub mod send_reply;
pub mod send_trust_center_link_key;
pub mod send_unicast;
pub mod set_address_table_remote_eui64;
pub mod set_address_table_remote_node_id;
pub mod set_beacon_classification_params;
pub mod set_binding;
pub mod set_binding_remote_node_id;
pub mod set_broken_route_error_code;
pub mod set_child_data;
pub mod set_concentrator;
pub mod set_configuration_value;
pub mod set_duty_cycle_limits_in_stack;
pub mod set_extended_timeout;
pub mod set_hub_connectivity;
pub mod set_initial_security_state;
pub mod set_key_table_entry;
pub mod set_logical_and_radio_channel;
pub mod set_logical_channel;
pub mod set_long_up_time;
pub mod set_mac_poll_failure_wait_time;
pub mod set_manufacturer_code;
pub mod set_mfg_token;
pub mod set_multicast_table_entry;
pub mod set_neighbor_frame_counter;
pub mod set_parent_classification_enabled;
pub mod set_passive_ack_config;
pub mod set_policy;
pub mod set_power_descriptor;
pub mod set_preinstalled_cbke_data;
pub mod set_radio_channel;
pub mod set_radio_ieee802154_cca_mode;
pub mod set_radio_power;
pub mod set_routing_shortcut_threshold;
pub mod set_security_key;
pub mod set_security_parameters;
pub mod set_source_route_discovery_mode;
pub mod set_timer;
pub mod set_token;
pub mod set_token_data;
pub mod set_value;
pub mod set_zll_additional_state;
pub mod set_zll_node_type;
pub mod set_zll_primary_channel_mask;
pub mod set_zll_secondary_channel_mask;
pub mod stack_status_handler;
pub mod stack_token_changed_handler;
pub mod start_scan;
pub mod stop_scan;
pub mod switch_network_key_handler;
pub mod timer_handler;
pub mod trust_center_join_handler;
pub mod unicast_current_network_key;
pub mod unicast_nwk_key_update;
pub mod unused_pan_id_found_handler;
pub mod update_tc_link_key;
pub mod version;
pub mod write_attribute;
pub mod write_node_data;
pub mod zigbee_key_establishment_handler;
pub mod zll_address_assignment_handler;
pub mod zll_clear_tokens;
pub mod zll_get_tokens;
pub mod zll_network_found_handler;
pub mod zll_network_ops;
pub mod zll_operation_in_progress;
pub mod zll_rx_on_when_idle_get_active;
pub mod zll_scan_complete_handler;
pub mod zll_set_data_token;
pub mod zll_set_initial_security_state;
pub mod zll_set_non_zll_network;
pub mod zll_set_radio_idle_mode;
pub mod zll_set_rx_on_when_idle;
pub mod zll_set_security_state_without_key;
pub mod zll_start_scan;
pub mod zll_touch_link_target_handler;

pub trait Parameter: Send + Sync + FromLeBytes + ToLeBytes {
    type Id: Copy
        + Debug
        + Display
        + Eq
        + Send
        + Sync
        + From<Control>
        + Into<Control>
        + FromLeBytes
        + ToLeBytes;
    const ID: Self::Id;
}

/// Possible callback responses, which are called "handler"s according to the EZSP documentation.
#[allow(clippy::large_enum_variant)]
pub enum Handler {
    BootloadTransmitComplete(bootload_transmit_complete_handler::Response),
    CalculateSmacs(calculate_smacs_handler::Response),
    CalculateSmacs283k1(calculate_smacs283k1::Response),
    // TODO: implement all.
}
