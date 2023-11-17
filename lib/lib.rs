#![allow(clippy::needless_return)]

pub mod duration;
pub mod errors;

use duration::Hhmmss;
use serde::{Deserialize, Serialize};
use serde_json::{self, Error};

pub const PACKET_SIZE: usize = 240;

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    // A rolling unique identifier for the current packet. Can be used to order and drop received packets.
    pub packet_uid: u64,

    // Time spent in game since boot.
    pub game_total_time: f32,

    // Time spent since the last frame.
    pub game_delta_time: f32,

    // Frame count in-game since boot.
    pub game_frame_count: u64,

    // For shift lights, from 0 ('vehicle_engine_rpm_current'='shiftlights_rpm_start') to 1 ('vehicle_engine_rpm_current'='shiftlights_rpm_end').
    pub shiftlights_fraction: f32,

    // Shift lights start at 'vehicle_engine_rpm_current' value.
    pub shiftlights_rpm_start: f32,

    // Shift lights end (i.e. optimal shift) at 'vehicle_engine_rpm_current' value.
    pub shiftlights_rpm_end: f32,

    // Are shift lights RPM data valid: 'vehicle_engine_rpm_current', 'shiftlights_rpm_start', 'shiftlights_rpm_end'
    pub shiftlights_rpm_valid: bool,

    // Gear index or value of 'vehicle_gear_index_neutral' or 'vehicle_gear_index_reverse'
    pub vehicle_gear_index: u8,

    // 'vehicle_gear_index' if the gearbox is Neutral.
    pub vehicle_gear_index_neutral: u8,

    // 'vehicle_gear_index' if the gearbox is Reverse.
    pub vehicle_gear_index_reverse: u8,

    // Number of forward gears.
    pub vehicle_gear_maximum: u8,

    // Car body speed.
    pub vehicle_speed: f32,

    // Car speed at wheel/road due to transmission (for speedo use). NB. May differ from 'vehicle_speed'.
    pub vehicle_transmission_speed: f32,

    // Car position X component, positive left.
    pub vehicle_position_x: f32,

    // Car position Y component, positive up.
    pub vehicle_position_y: f32,

    // Car position Z component, positive forward.
    pub vehicle_position_z: f32,

    // Car velocity X component, positive left.
    pub vehicle_velocity_x: f32,

    // Car velocity Y component, positive up.
    pub vehicle_velocity_y: f32,

    // Car velocity Z component, positive forward.
    pub vehicle_velocity_z: f32,

    // Car acceleration X component, positive left.
    pub vehicle_acceleration_x: f32,

    // Car acceleration Y component, positive up.
    pub vehicle_acceleration_y: f32,

    // Car acceleration Z component, positive forward.
    pub vehicle_acceleration_z: f32,

    // Car left unit vector X component, positive left.
    pub vehicle_left_direction_x: f32,

    // Car left unit vector Y component, positive up.
    pub vehicle_left_direction_y: f32,

    // Car left unit vector Z component, positive forward.
    pub vehicle_left_direction_z: f32,

    // Car forward unit vector X component, positive left.
    pub vehicle_forward_direction_x: f32,

    // Car forward unit vector Y component, positive up.
    pub vehicle_forward_direction_y: f32,

    // Car forward unit vector Z component, positive forward.
    pub vehicle_forward_direction_z: f32,

    // Car up unit vector X component, positive left.
    pub vehicle_up_direction_x: f32,

    // Car up unit vector Y component, positive up.
    pub vehicle_up_direction_y: f32,

    // Car up unit vector Z component, positive forward.
    pub vehicle_up_direction_z: f32,

    // Wheel hub height displacement, back left, positive up.
    pub vehicle_hub_position_bl: f32,

    // Wheel hub height displacement, back right, positive up.
    pub vehicle_hub_position_br: f32,

    // Wheel hub height displacement, front left, positive up.
    pub vehicle_hub_position_fl: f32,

    // Wheel hub height displacement, front right, positive up.
    pub vehicle_hub_position_fr: f32,

    // Wheel hub vertical velocity, back left, positive up.
    pub vehicle_hub_velocity_bl: f32,

    // Wheel hub vertical velocity, back right, positive up.
    pub vehicle_hub_velocity_br: f32,

    // Wheel hub vertical velocity, front left, positive up.
    pub vehicle_hub_velocity_fl: f32,

    // Wheel hub vertical velocity, front right, positive up.
    pub vehicle_hub_velocity_fr: f32,

    // Contact patch forward speed, back left.
    pub vehicle_cp_forward_speed_bl: f32,

    // Contact patch forward speed, back right.
    pub vehicle_cp_forward_speed_br: f32,

    // Contact patch forward speed, front left.
    pub vehicle_cp_forward_speed_fl: f32,

    // Contact patch forward speed, front right.
    pub vehicle_cp_forward_speed_fr: f32,

    // Brake temperature, back left.
    pub vehicle_brake_temperature_bl: f32,

    // Brake temperature, back right.
    pub vehicle_brake_temperature_br: f32,

    // Brake temperature, front left.
    pub vehicle_brake_temperature_fl: f32,

    // Brake temperature, front right.
    pub vehicle_brake_temperature_fr: f32,

    // Engine rotation rate, maximum.
    pub vehicle_engine_rpm_max: f32,

    // Engine rotation rate, at idle.
    pub vehicle_engine_rpm_idle: f32,

    // Engine rotation rate, current.
    pub vehicle_engine_rpm_current: f32,

    // Throttle pedal after assists and overrides, 0 (off) to 1 (full).
    pub vehicle_throttle: f32,

    // Brake pedal after assists and overrides, 0 (off) to 1 (full).
    pub vehicle_brake: f32,

    // Clutch pedal after assists and overrides, 0 (off) to 1 (full).
    pub vehicle_clutch: f32,

    // Steering after assists and overrides, -1 (full left) to 1 (full right).
    pub vehicle_steering: f32,

    // Handbrake after assists and overrides, 0 (off) to 1 (full).
    pub vehicle_handbrake: f32,

    // Time spent on the current stage.
    pub stage_current_time: f32,

    // Distance reached on the current stage.
    pub stage_current_distance: f64,

    // Total length of the current stage.
    pub stage_length: f64,
}

// Default trait implementation for Packet
impl Default for Packet {
    fn default() -> Self {
        Self {
            packet_uid: 0,
            game_total_time: 0.0,
            game_delta_time: 0.0,
            game_frame_count: 0,
            shiftlights_fraction: 0.0,
            shiftlights_rpm_start: 0.0,
            shiftlights_rpm_end: 0.0,
            shiftlights_rpm_valid: false,
            vehicle_gear_index: 0,
            vehicle_gear_index_neutral: 0,
            vehicle_gear_index_reverse: 0,
            vehicle_gear_maximum: 0,
            vehicle_speed: 0.0,
            vehicle_transmission_speed: 0.0,
            vehicle_position_x: 0.0,
            vehicle_position_y: 0.0,
            vehicle_position_z: 0.0,
            vehicle_velocity_x: 0.0,
            vehicle_velocity_y: 0.0,
            vehicle_velocity_z: 0.0,
            vehicle_acceleration_x: 0.0,
            vehicle_acceleration_y: 0.0,
            vehicle_acceleration_z: 0.0,
            vehicle_left_direction_x: 0.0,
            vehicle_left_direction_y: 0.0,
            vehicle_left_direction_z: 0.0,
            vehicle_forward_direction_x: 0.0,
            vehicle_forward_direction_y: 0.0,
            vehicle_forward_direction_z: 0.0,
            vehicle_up_direction_x: 0.0,
            vehicle_up_direction_y: 0.0,
            vehicle_up_direction_z: 0.0,
            vehicle_hub_position_bl: 0.0,
            vehicle_hub_position_br: 0.0,
            vehicle_hub_position_fl: 0.0,
            vehicle_hub_position_fr: 0.0,
            vehicle_hub_velocity_bl: 0.0,
            vehicle_hub_velocity_br: 0.0,
            vehicle_hub_velocity_fl: 0.0,
            vehicle_hub_velocity_fr: 0.0,
            vehicle_cp_forward_speed_bl: 0.0,
            vehicle_cp_forward_speed_br: 0.0,
            vehicle_cp_forward_speed_fl: 0.0,
            vehicle_cp_forward_speed_fr: 0.0,
            vehicle_brake_temperature_bl: 0.0,
            vehicle_brake_temperature_br: 0.0,
            vehicle_brake_temperature_fl: 0.0,
            vehicle_brake_temperature_fr: 0.0,
            vehicle_engine_rpm_max: 0.0,
            vehicle_engine_rpm_idle: 0.0,
            vehicle_engine_rpm_current: 0.0,
            vehicle_throttle: 0.0,
            vehicle_brake: 0.0,
            vehicle_clutch: 0.0,
            vehicle_steering: 0.0,
            vehicle_handbrake: 0.0,
            stage_current_time: 0.0,
            stage_current_distance: 0.0,
            stage_length: 0.0,
        }
    }
}

impl Packet {
    pub fn to_json(self) -> Result<String, Error> {
        serde_json::to_string(&self)
    }

    /// Returns the current stage time as a prettified string. (00:15:24.497)
    pub fn stage_time_pretty(self) -> String {
        let dur = chrono::Duration::seconds(self.stage_current_time as i64);
        return dur.hhmmssxxx();
    }
    /// Returns the total ingame time as a prettified string. (00:15:24.497)
    pub fn total_time_pretty(self) -> String {
        let dur = chrono::Duration::seconds(self.game_total_time as i64);
        return dur.hhmmssxxx();
    }
    /// Returns current throttle input as a percentage.
    pub fn throttle_percentage(self) -> usize {
        return self.vehicle_throttle as usize * 100;
    }
    /// Returns current brake input as a percentage.
    pub fn brake_percentage(self) -> usize {
        return self.vehicle_brake as usize * 100;
    }
    /// Returns current clutch input as a percentage.
    pub fn clutch_percentage(self) -> usize {
        return self.vehicle_clutch as usize * 100;
    }
    /// Returns current handbrake input as a percentage.
    pub fn handbrake_percentage(self) -> usize {
        return self.vehicle_handbrake as usize * 100;
    }
}

impl TryFrom<&[u8; 240]> for Packet {
    type Error = bincode::Error;
    fn try_from(value: &[u8; PACKET_SIZE]) -> Result<Self, bincode::Error> {
        return bincode::deserialize(value);
    }
}
