use std::collections::HashMap;

use super::event::EventDirection;

lazy_static::lazy_static! {
 pub static ref SENSOR_GENERIC_EVENT_DESC: HashMap<u32, (&'static str,EventDirection)>  = {
  HashMap::from([
    // Event_Type, Offset
    ((0x01 << 8),        ("Lower_Non_critical_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x01, ("Lower_Non_critical_going_high",EventDirection::Deassert)),
    ((0x01 << 8) | 0x02, ("Lower_Critical_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x03, ("Lower_Critical_going_high",EventDirection::Deassert)),
    ((0x01 << 8) | 0x04, ("Lower_Non_recoverable_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x05, ("Lower_Non_recoverable_going_high",EventDirection::Deassert)),
    ((0x01 << 8) | 0x06, ("Upper_Non_critical_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x07, ("Upper_Non_critical_going_high",EventDirection::Deassert)),
    ((0x01 << 8) | 0x08, ("Upper_Critical_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x09, ("Upper_Critical_going_high",EventDirection::Deassert)),
    ((0x01 << 8) | 0x0a, ("Upper_Non_recoverable_going_low",EventDirection::Deassert)),
    ((0x01 << 8) | 0x0b, ("Upper_Non_recoverable_going_high",EventDirection::Deassert)),
    ((0x02 << 8),        ("Transition_to_Idle",EventDirection::Assert)),
    ((0x02 << 8) | 0x01, ("Transition_to_Active",EventDirection::Assert)),
    ((0x02 << 8) | 0x02, ("Transition_to_Busy",EventDirection::Deassert)),
    ((0x03 << 8),        ("State_Deasserted",EventDirection::Deassert)),
    ((0x03 << 8) | 0x01, ("State_Asserted",EventDirection::Assert)),
    ((0x04 << 8),        ("Predictive_Failure_deasserted",EventDirection::Deassert)),
    ((0x04 << 8) | 0x01, ("Predictive_Failure_asserted",EventDirection::Assert)),
    ((0x05 << 8),        ("Limit_Not_Exceeded",EventDirection::Assert)),
    ((0x05 << 8) | 0x01, ("Limit_Exceeded",EventDirection::Deassert)),
    ((0x06 << 8),        ("Performance_Met",EventDirection::Assert)),
    ((0x06 << 8) | 0x01, ("Performance_Lags",EventDirection::Deassert)),
    ((0x07 << 8),        ("transition_to_OK",EventDirection::Assert)),
    ((0x07 << 8) | 0x01, ("transition_to_Non_Critical_from_OK",EventDirection::Assert)),
    ((0x07 << 8) | 0x02, ("transition_to_Critical_from_less_severe",EventDirection::Assert)),
    ((0x07 << 8) | 0x03, ("transition_to_Non_recoverable_from_less_severe",EventDirection::Assert)),
    ((0x07 << 8) | 0x04, ("transition_to_Non_Critical_from_more_severe",EventDirection::Assert)),
    ((0x07 << 8) | 0x05, ("transition_to_Critical_from_Non_recoverable",EventDirection::Assert)),
    ((0x07 << 8) | 0x06, ("transition_to_Non_recoverable",EventDirection::Assert)),
    ((0x07 << 8) | 0x07, ("Monitor",EventDirection::Assert)),
    ((0x07 << 8) | 0x08, ("Informational",EventDirection::Assert)),
    ((0x08 << 8),        ("Device_Removed_Device_Absent",EventDirection::Deassert)),
    ((0x08 << 8) | 0x01, ("Device_Inserted_Device_Present",EventDirection::Assert)),
    ((0x09 << 8),        ("Device_Disabled",EventDirection::Deassert)),
    ((0x09 << 8) | 0x01, ("Device_Enabled",EventDirection::Assert)),
    ((0x0a << 8),        ("transition_to_Running",EventDirection::Assert)),
    ((0x0a << 8) | 0x01, ("transition_to_In_Test",EventDirection::Assert)),
    ((0x0a << 8) | 0x02, ("transition_to_Power_Off",EventDirection::Deassert)),
    ((0x0a << 8) | 0x03, ("transition_to_On_Line",EventDirection::Assert)),
    ((0x0a << 8) | 0x04, ("transition_to_Off_Line",EventDirection::Deassert)),
    ((0x0a << 8) | 0x05, ("transition_to_Off_Duty",EventDirection::Deassert)),
    ((0x0a << 8) | 0x06, ("transition_to_Degraded",EventDirection::Deassert)),
    ((0x0a << 8) | 0x07, ("transition_to_Power_Save",EventDirection::Deassert)),
    ((0x0a << 8) | 0x08, ("install_Error",EventDirection::Deassert)),
    ((0x0b << 8),        ("Fully_Redundant___formerly_Redundancy_Regained)",EventDirection::Assert)),
    ((0x0b << 8) | 0x01, ("Redundancy_Lost",EventDirection::Deassert)),
    ((0x0b << 8) | 0x02, ("Redundancy_Degraded",EventDirection::Deassert)),
    ((0x0b << 8) | 0x03, ("Non_redundant_Sufficient_Resources_from_Redundant",EventDirection::Deassert)),
    ((0x0b << 8) | 0x04, ("Non_redundant_Sufficient_Resources_from_Insufficient_Resources",EventDirection::Deassert)),
    ((0x0b << 8) | 0x05, ("Non_redundant_Insufficient_Resources",EventDirection::Assert)),
    ((0x0b << 8) | 0x06, ("Redundancy_Degraded_from_Fully_Redundant",EventDirection::Assert)),
    ((0x0b << 8) | 0x07, ("Redundancy_Degraded_from_Non_redundant",EventDirection::Assert)),
    ((0x0c << 8),        ("D0 Power_State",EventDirection::Deassert)),
    ((0x0c << 8) | 0x01, ("D1 Power_State",EventDirection::Deassert)),
    ((0x0c << 8) | 0x02, ("D2 Power_State",EventDirection::Deassert)),
    ((0x0c << 8) | 0x03, ("D3 Power_State",EventDirection::Deassert)),
  ])
};

 pub static ref SENSOR_SPECIFIC_EVENT_DESC: HashMap<u32, (&'static str, EventDirection)> = {
    HashMap::from([
    // Sensor_Type, Offset, Event_Data2, Event_Data3
    ((0x05 << 24) | (0xff << 8) | 0xff,                ("General_Chassis_Intrusion", EventDirection::Deassert)),
    ((0x05 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Drive_Bay_intrusion", EventDirection::Deassert)),
    ((0x05 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("I_O_Card_area_intrusion", EventDirection::Deassert)),
    ((0x05 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Processor_area_intrusion", EventDirection::Deassert)),
    ((0x05 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("LAN_Leash_Lost___system_is_unplugged_from_LAN)", EventDirection::Deassert)),
    ((0x05 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Unauthorized_dock", EventDirection::Deassert)),
    ((0x05 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("FAN_area_intrusion___supports_detection_of_hot_plug_fan_tampering)", EventDirection::Deassert)),
    ((0x06 << 24) | (0xff << 8) | 0xff,                ("Secure_Mode___Front_Panel_Lockout___Violation_attempt", EventDirection::Deassert)),
    ((0x06 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Pre_boot_Password_Violation_user_password", EventDirection::Deassert)),
    ((0x06 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Pre_boot_Password_Violation_attempt_setup_password", EventDirection::Deassert)),
    ((0x06 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Pre_boot_Password_Violation_network_boot_password", EventDirection::Deassert)),
    ((0x06 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Other_pre_boot_Password_Violation", EventDirection::Deassert)),
    ((0x06 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Out_of_band_Access_Password_Violation", EventDirection::Deassert)),
    ((0x07 << 24) | (0xff << 8) | 0xff,                ("IERR", EventDirection::Deassert)),
    ((0x07 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Thermal_Trip", EventDirection::Deassert)),
    ((0x07 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("FRB1_BIST_failure", EventDirection::Deassert)),
    ((0x07 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("FRB2_Hang_in_POST_failure", EventDirection::Deassert)),
    ((0x07 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("FRB3_Processor_Startup_Initialization_failure", EventDirection::Deassert)),
    ((0x07 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Configuration_Error", EventDirection::Deassert)),
    ((0x07 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("SM_BIOS_Uncorrectable_CPU_complex_Error", EventDirection::Deassert)),
    ((0x07 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Processor_Presence_detected", EventDirection::Assert)),
    ((0x07 << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Processor_disabled", EventDirection::Deassert)),
    ((0x07 << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("Terminator_Presence_Detected", EventDirection::Assert)),
    ((0x07 << 24) | (0x0a << 16) | (0xff << 8) | 0xff, ("Processor_Automatically_Throttled", EventDirection::Assert)),
    ((0x07 << 24) | (0x0b << 16) | (0xff << 8) | 0xff, ("Machine_Check_Exception___Uncorrectable)", EventDirection::Deassert)),
    ((0x07 << 24) | (0x0c << 16) | (0xff << 8) | 0xff, ("Correctable_Machine_Check_Error", EventDirection::Deassert)),
    ((0x08 << 24) | (0xff << 8) | 0xff,                ("Presence_detected", EventDirection::Assert)),
    ((0x08 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Power_Supply_Failure_detected", EventDirection::Deassert)),
    ((0x08 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Predictive_Failure", EventDirection::Deassert)),
    ((0x08 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Power_Supply_input_lost___AC_DC)", EventDirection::Deassert)),
    ((0x08 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Power_Supply_input_lost_or_out_of_range", EventDirection::Deassert)),
    ((0x08 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Power_Supply_input_out_of_range_but_present", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8),        ("Configuration_error__Vendor_mismatch", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8) | 0x01, ("Configuration_error__Revision_mismatch", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8) | 0x02, ("Configuration_error__Processor_missing", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8) | 0x03, ("Configuration_error__Power_Supply_rating_mismatch", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8) | 0x04, ("Configuration_error__Voltage_rating_mismatch", EventDirection::Deassert)),
    ((0x08 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Configuration_error", EventDirection::Deassert)),
    ((0x08 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Power_Supply_Inactive___in_standby_state)", EventDirection::Deassert)),
    ((0x09 << 24) | (0xff << 8) | 0xff,                ("Power_Off_or_Power_Down", EventDirection::Deassert)),
    ((0x09 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Power_Cycle", EventDirection::Deassert)),
    ((0x09 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("240VA_Power_Down", EventDirection::Deassert)),
    ((0x09 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Interlock_Power_Down", EventDirection::Deassert)),
    ((0x09 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("AC_lost_or_Power_input_lost", EventDirection::Deassert)),
    ((0x09 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Soft_Power_Control_Failure", EventDirection::Deassert)),
    ((0x09 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Power_Unit_Failure_detected", EventDirection::Deassert)),
    ((0x09 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Predictive_Failure", EventDirection::Deassert)),
    ((0x0c << 24) | (0xff << 8) | 0xff,                ("Correctable_ECC_or_other_correctable_memory_error", EventDirection::Deassert)),
    ((0x0c << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Uncorrectable_ECC_or_other_uncorrectable_memory_error", EventDirection::Deassert)),
    ((0x0c << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Parity", EventDirection::Deassert)),
    ((0x0c << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Memory_Scrub_Failed___stuck_bit)", EventDirection::Deassert)),
    ((0x0c << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Memory_Device_Disabled", EventDirection::Deassert)),
    ((0x0c << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Correctable_ECC_or_other_correctable_memory_error_logging_limit_reached", EventDirection::Deassert)),
    ((0x0c << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Presence_detected", EventDirection::Assert)),
    ((0x0c << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Configuration_error", EventDirection::Deassert)),
    ((0x0c << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Spare", EventDirection::Deassert)),
    ((0x0c << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("Memory_Automatically_Throttled", EventDirection::Deassert)),
    ((0x0c << 24) | (0x0a << 16) | (0xff << 8) | 0xff, ("Critical_Overtemperature", EventDirection::Deassert)),
    ((0x0d << 24) | (0xff << 8) | 0xff,                ("Drive_Presence", EventDirection::Assert)),
    ((0x0d << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Drive_Fault", EventDirection::Deassert)),
    ((0x0d << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Predictive_Failure", EventDirection::Deassert)),
    ((0x0d << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Hot_Spare", EventDirection::Deassert)),
    ((0x0d << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Consistency_Check_or_Parity_Check_in_progress", EventDirection::Deassert)),
    ((0x0d << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("In_Critical_Array", EventDirection::Deassert)),
    ((0x0d << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("In_Failed_Array", EventDirection::Deassert)),
    ((0x0d << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Rebuild_Remap_in_progress", EventDirection::Deassert)),
    ((0x0d << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Rebuild_Remap_Aborted___was_not_completed_normally)", EventDirection::Deassert)),
    ((0x0f << 24) | 0xff,                              ("System_Firmware_Error__Unspecified", EventDirection::Deassert)),
    ((0x0f << 24) | (0x01 << 8) | 0xff,                ("System_Firmware_Error__No_system_memory_is_physically_installed_in_the_system", EventDirection::Deassert)),
    ((0x0f << 24) | (0x02 << 8) | 0xff,                ("System_Firmware_Error__No_usable_system_memory", EventDirection::Deassert)),
    ((0x0f << 24) | (0x03 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_hard_disk_ATAPI_IDE_device_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x04 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_system_board_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x05 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_diskettesubsystem_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x06 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_hard_ disk_controller_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x07 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_PS_2 or_USB_keyboard_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x08 << 8) | 0xff,                ("System_Firmware_Error__Removable_boot_media_not),found", EventDirection::Deassert)),
    ((0x0f << 24) | (0x09 << 8) | 0xff,                ("System_Firmware_Error__Unrecoverable_video_controller_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0x0a << 8) | 0xff,                ("System_Firmware_Error__No_video_device_detected", EventDirection::Deassert)),
    ((0x0f << 24) | (0x0b << 8) | 0xff,                ("System_Firmware_Error__Firmware___BIOS___ROM_corruption_detected", EventDirection::Deassert)),
    ((0x0f << 24) | (0x0c << 8) | 0xff,                ("System_Firmware_Error__CPU_voltage_mismatch", EventDirection::Deassert)),
    ((0x0f << 24) | (0x0d << 8) | 0xff,                ("System_Firmware_Error__CPU_speed_matching_failure", EventDirection::Deassert)),
    ((0x0f << 24) | (0xff << 8) | 0xff,                ("System_Firmware_Error", EventDirection::Deassert)),
    ((0x0f << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("System_Firmware_Hang", EventDirection::Deassert)),
    ((0x0f << 24) | (0x02 << 16) | 0xff,               ("System_Firmware_Progress__Unspecified", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x01 << 8) | 0xff, ("System_Firmware_Progress__Memory_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x02 << 8) | 0xff, ("System_Firmware_Progress__Hard_disk_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x03 << 8) | 0xff, ("System_Firmware_Progress__Secondary_processors_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x04 << 8) | 0xff, ("System_Firmware_Progress__User_authentication", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x05 << 8) | 0xff, ("System_Firmware_Progress__User_initiated_system___setup", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x06 << 8) | 0xff, ("System_Firmware_Progress__USB_resource_configuration", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x07 << 8) | 0xff, ("System_Firmware_Progress__PCI_resource_configuration", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x08 << 8) | 0xff, ("System_Firmware_Progress__Option_ROM_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x09 << 8) | 0xff, ("System_Firmware_Progress__Video_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0a << 8) | 0xff, ("System_Firmware_Progress__Cache_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0b << 8) | 0xff, ("System_Firmware_Progress__SM_Bus_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0c << 8) | 0xff, ("System_Firmware_Progress__Keyboard_controller_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0d << 8) | 0xff, ("System_Firmware_Progress__Embedded_controller_management_controlle_EventDirection___Assertr___initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0e << 8) | 0xff, ("System_Firmware_Progress__Docking_station_attachment", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x0f << 8) | 0xff, ("System_Firmware_Progress__Enabling_docking_station", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x10 << 8) | 0xff, ("System_Firmware_Progress__Docking_station_ejection", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x11 << 8) | 0xff, ("System_Firmware_Progress__Disabling_docking_station", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x12 << 8) | 0xff, ("System_Firmware_Progress__Calling_operating_system_wake_up_vector", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x13 << 8) | 0xff, ("System_Firmware_Progress__Starting_operating_system_boot_process", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x14 << 8) | 0xff, ("System_Firmware_Progress__Baseboard_or_motherboard_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x15 << 8) | 0xff, ("System_Firmware_Progress__reserved", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x16 << 8) | 0xff, ("System_Firmware_Progress__Floppy_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x17 << 8) | 0xff, ("System_Firmware_Progress__Keyboard_test", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x18 << 8) | 0xff, ("System_Firmware_Progress__Pointing_device_test", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0x19 << 8) | 0xff, ("System_Firmware_Progress__Primary_processor_initialization", EventDirection::Assert)),
    ((0x0f << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("System_Firmware_Progress", EventDirection::Assert)),
    ((0x10 << 24) | (0xff << 8) | 0xff,                ("Correctable_Memory_Error_Logging_Disabled", EventDirection::Deassert)),
    ((0x10 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Event_Type_Logging_Disabled", EventDirection::Deassert)),
    ((0x10 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Log_Area_Reset_Cleared", EventDirection::Deassert)),
    ((0x10 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("All_Event_Logging_Disabled", EventDirection::Deassert)),
    ((0x10 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("SEL_Full", EventDirection::Deassert)),
    ((0x10 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("SEL_Almost_Full", EventDirection::Deassert)),
    ((0x10 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Correctable_Machine_Check_Error_Logging_Disabled", EventDirection::Deassert)),
    ((0x11 << 24) | (0xff << 8) | 0xff,                ("BIOS_Watchdog_Reset", EventDirection::Deassert)),
    ((0x11 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_Reset", EventDirection::Deassert)),
    ((0x11 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_Shut_Down", EventDirection::Deassert)),
    ((0x11 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_Power_Down", EventDirection::Deassert)),
    ((0x11 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_Power_Cycle", EventDirection::Deassert)),
    ((0x11 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_NMI_or_Diagnostic_Interrupt", EventDirection::Deassert)),
    ((0x11 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_Expired_status_only", EventDirection::Deassert)),
    ((0x11 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("OS_Watchdog_pre_timeout_Interrupt_non_NMI", EventDirection::Deassert)),
    ((0x12 << 24) | (0xff << 8) | 0xff,                ("System_Reconfigured", EventDirection::Deassert)),
    ((0x12 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("OEM_System_Boot_Event", EventDirection::Deassert)),
    ((0x12 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Undetermined_system_hardware_failure", EventDirection::Deassert)),
    ((0x12 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Entry_added_to_Auxiliary_Log", EventDirection::Deassert)),
    ((0x12 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("PEF_Action", EventDirection::Deassert)),
    ((0x12 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Timestamp_Clock_Synch", EventDirection::Assert)),
    ((0x13 << 24) | (0xff << 8) | 0xff,                ("Front_Panel_NMI_or_Diagnostic_Interrupt", EventDirection::Deassert)),
    ((0x13 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Bus_Timeout", EventDirection::Deassert)),
    ((0x13 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("I_O_channel_check_NMI", EventDirection::Deassert)),
    ((0x13 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Software_NMI", EventDirection::Deassert)),
    ((0x13 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("PCI_PERR", EventDirection::Deassert)),
    ((0x13 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("PCI_SERR", EventDirection::Deassert)),
    ((0x13 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("EISA_Fail_Safe_Timeout", EventDirection::Deassert)),
    ((0x13 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Bus_Correctable_Error", EventDirection::Deassert)),
    ((0x13 << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Bus_Uncorrectable_Error", EventDirection::Deassert)),
    ((0x13 << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("Fatal_NMI", EventDirection::Deassert)),
    ((0x13 << 24) | (0x0a << 16) | (0xff << 8) | 0xff, ("Bus_Fatal_Error", EventDirection::Deassert)),
    ((0x13 << 24) | (0x0b << 16) | (0xff << 8) | 0xff, ("Bus_Degraded", EventDirection::Deassert)),
    ((0x14 << 24) | (0xff << 8) | 0xff,                ("Power_Button_pressed", EventDirection::Deassert)),
    ((0x14 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Sleep_Button_pressed", EventDirection::Deassert)),
    ((0x14 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Reset_Button_pressed", EventDirection::Deassert)),
    ((0x14 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("FRU_latch_open", EventDirection::Deassert)),
    ((0x14 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("FRU_service_request_button", EventDirection::Deassert)),
    ((0x19 << 24) | (0xff << 8) | 0xff,                ("Soft_Power_Control_Failure", EventDirection::Deassert)),
    ((0x19 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Thermal_Trip", EventDirection::Deassert)),
    ((0x1b << 24) | (0xff << 8) | 0xff,                ("Cable_Interconnect_is_connected", EventDirection::Assert)),
    ((0x1b << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Configuration_Error", EventDirection::Assert)),
    ((0x1d << 24) | (0xff << 8) | 0xff,                ("Initiated_by_power_up", EventDirection::Deassert)),
    ((0x1d << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Initiated_by_hard_reset", EventDirection::Deassert)),
    ((0x1d << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Initiated_by_warm_reset", EventDirection::Deassert)),
    ((0x1d << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("User_requested_PXE_boot", EventDirection::Deassert)),
    ((0x1d << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Automatic_boot_to_diagnostic", EventDirection::Deassert)),
    ((0x1d << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("OS_or_run_time_software_initiated_hard_reset", EventDirection::Deassert)),
    ((0x1d << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("OS_or_run_time_software_initiated_warm_reset", EventDirection::Deassert)),
    ((0x1d << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("System_Restart", EventDirection::Deassert)),
    ((0x1e << 24) | (0xff << 8) | 0xff,                ("No_bootable_media", EventDirection::Deassert)),
    ((0x1e << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Non_bootable_diskette_left_in_drive", EventDirection::Deassert)),
    ((0x1e << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("PXE_Server_not_found", EventDirection::Deassert)),
    ((0x1e << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Invalid_boot_sector", EventDirection::Deassert)),
    ((0x1e << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Timeout_waiting_for_user_selection_of_boot_source", EventDirection::Deassert)),
    ((0x1f << 24) | (0xff << 8) | 0xff,                ("A__boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("C__boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("PXE_boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Diagnostic_boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("CD_ROM_boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("ROM_boot_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("boot_completed_boot_device_not_specified", EventDirection::Deassert)),
    ((0x1f << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Base_OS_Hypervisor_Installation_started", EventDirection::Assert)),
    ((0x1f << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Base_OS_Hypervisor_Installation_completed", EventDirection::Assert)),
    ((0x1f << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("Base_OS_Hypervisor_Installation_aborted", EventDirection::Deassert)),
    ((0x1f << 24) | (0x0a << 16) | (0xff << 8) | 0xff, ("Base_OS_Hypervisor_Installation_failed", EventDirection::Deassert)),
    ((0x20 << 24) | (0xff << 8) | 0xff,                ("Critical_stop_during_OS_load_or_initialization", EventDirection::Deassert)),
    ((0x20 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Run_time_Critical_Stop", EventDirection::Deassert)),
    ((0x20 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("OS_Graceful_Stop", EventDirection::Deassert)),
    ((0x20 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("OS_Graceful_Shutdown", EventDirection::Deassert)),
    ((0x20 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Soft_Shutdown_initiated_by_PEF", EventDirection::Deassert)),
    ((0x20 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Agent_Not_Responding", EventDirection::Deassert)),
    ((0x21 << 24) | (0xff << 8) | 0xff,                ("Fault_Status_asserted", EventDirection::Deassert)),
    ((0x21 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Identify_Status_asserted", EventDirection::Assert)),
    ((0x21 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Slot_Connector_Device_installed_attached", EventDirection::Assert)),
    ((0x21 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Slot_Connector_Ready_for_Device_Installation", EventDirection::Assert)),
    ((0x21 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Slot_Connector_Ready_for_Device_Removal", EventDirection::Assert)),
    ((0x21 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Slot_Power_is_Off", EventDirection::Assert)),
    ((0x21 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Slot_Connector_Device_Removal_Request", EventDirection::Assert)),
    ((0x21 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Interlock_asserted", EventDirection::Assert)),
    ((0x21 << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Slot_is_Disabled", EventDirection::Deassert)),
    ((0x21 << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("Slot_holds_spare_device", EventDirection::Assert)),
    ((0x22 << 24) | (0xff << 8) | 0xff,                ("S0_G0_working", EventDirection::Assert)),
    ((0x22 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("S1_sleeping_with_system_h_w_and_processor_context_maintained", EventDirection::Assert)),
    ((0x22 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("S2_sleeping_processor_context_lost", EventDirection::Assert)),
    ((0x22 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("S3_sleeping_processor_and_h_w_context_lost_memory_retained", EventDirection::Assert)),
    ((0x22 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("S4_non_volatile_sleep_or_suspend_to_disk", EventDirection::Assert)),
    ((0x22 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("S5_G2_soft_off", EventDirection::Assert)),
    ((0x22 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("S4_S5_soft_off_particular_S4_or_S5_state_cannot_be_determined", EventDirection::Assert)),
    ((0x22 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("G3_Mechanical_Off", EventDirection::Assert)),
    ((0x22 << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Sleeping_in_an_S1_S2_or_S3_states", EventDirection::Assert)),
    ((0x22 << 24) | (0x09 << 16) | (0xff << 8) | 0xff, ("G1_sleeping", EventDirection::Assert)),
    ((0x22 << 24) | (0x0a << 16) | (0xff << 8) | 0xff, ("S5_entered_by_override", EventDirection::Assert)),
    ((0x22 << 24) | (0x0b << 16) | (0xff << 8) | 0xff, ("Legacy_ON_state", EventDirection::Assert)),
    ((0x22 << 24) | (0x0c << 16) | (0xff << 8) | 0xff, ("Legacy_OFF_state", EventDirection::Assert)),
    ((0x22 << 24) | (0x0e << 16) | (0xff << 8) | 0xff, ("Unknown", EventDirection::Assert)),
    ((0x23 << 24) | (0xff << 8) | 0xff,                ("Timer_expired, status_only", EventDirection::Assert)),
    ((0x23 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Hard_Reset", EventDirection::Deassert)),
    ((0x23 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Power_Down", EventDirection::Deassert)),
    ((0x23 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Power_Cycle", EventDirection::Deassert)),
    ((0x23 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("reserved", EventDirection::Deassert)),
    ((0x23 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("reserved", EventDirection::Deassert)),
    ((0x23 << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("reserved", EventDirection::Deassert)),
    ((0x23 << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("reserved", EventDirection::Deassert)),
    ((0x23 << 24) | (0x08 << 16) | (0xff << 8) | 0xff, ("Timer_interrupt", EventDirection::Deassert)),
    ((0x24 << 24) | (0xff << 8) | 0xff,                ("platform_generated_page", EventDirection::Assert)),
    ((0x24 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("platform_generated_LAN_alert", EventDirection::Deassert)),
    ((0x24 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Platform_Event_Trap_generated", EventDirection::Deassert)),
    ((0x24 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("platform_generated_SNMP_trap_OEM_format", EventDirection::Assert)),
    ((0x25 << 24) | (0xff << 8) | 0xff,                ("Entity_Present", EventDirection::Assert)),
    ((0x25 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Entity_Absent", EventDirection::Deassert)),
    ((0x25 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Entity_Disabled", EventDirection::Deassert)),
    ((0x27 << 24) | (0xff << 8) | 0xff,                ("LAN_Heartbeat_Lost", EventDirection::Deassert)),
    ((0x27 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("LAN_Heartbeat", EventDirection::Assert)),
    ((0x28 << 24) | (0xff << 8) | 0xff,                ("sensor_access_degraded_or_unavailable", EventDirection::Deassert)),
    ((0x28 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("controller_access_degraded_or_unavailable", EventDirection::Deassert)),
    ((0x28 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("management_controller_off_line", EventDirection::Deassert)),
    ((0x28 << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("management_controller_unavailable", EventDirection::Deassert)),
    ((0x28 << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("sensor_failure", EventDirection::Deassert)),
    ((0x28 << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("FRU_failure", EventDirection::Deassert)),
    ((0x29 << 24) | (0xff << 8) | 0xff,                ("battery_low", EventDirection::Deassert)),
    ((0x29 << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("battery_failed", EventDirection::Deassert)),
    ((0x29 << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("battery_presence_detected", EventDirection::Assert)),
    ((0x2a << 24) | (0xff << 8) | 0xff,                ("Session_Activated", EventDirection::Assert)),
    ((0x2a << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Session_Deactivated", EventDirection::Assert)),
    ((0x2a << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Invalid_Username_or_Password", EventDirection::Deassert)),
    ((0x2a << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Invalid_Password_Disable", EventDirection::Deassert)),
    ((0x2b << 24) | (0xff << 8) | 0xff,                ("Hardware_change_detected_with_associated_Entity", EventDirection::Deassert)),
    ((0x2b << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("Firmware_or_software_change_detected_with_associated_Entity", EventDirection::Deassert)),
    ((0x2b << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("Hardware_incompatibility_detected_with_associated_Entity", EventDirection::Deassert)),
    ((0x2b << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("Firmware_or_software_incompatibility_detected_with_associated_Entity", EventDirection::Deassert)),
    ((0x2b << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("Entity_is_of_an_invalid_or_unsupported_hardware_version", EventDirection::Deassert)),
    ((0x2b << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("Entity_contains_an_invalid_or_unsupported_firmware_or_software_version", EventDirection::Deassert)),
    ((0x2b << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("Hardware_Change_detected_with_associated_Entity_was_successful", EventDirection::Assert)),
    ((0x2b << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("Software_or_F_W_Change_detected_with_associated_Entity_was_successful", EventDirection::Assert)),
    ((0x2c << 24) | (0xff << 8) | 0xff,                ("FRU_Not_Installed", EventDirection::Deassert)),
    ((0x2c << 24) | (0x01 << 16) | (0xff << 8) | 0xff, ("FRU_Inactive", EventDirection::Deassert)),
    ((0x2c << 24) | (0x02 << 16) | (0xff << 8) | 0xff, ("FRU_Activation_Requested", EventDirection::Assert)),
    ((0x2c << 24) | (0x03 << 16) | (0xff << 8) | 0xff, ("FRU_Activation_In_Progress", EventDirection::Assert)),
    ((0x2c << 24) | (0x04 << 16) | (0xff << 8) | 0xff, ("FRU_Active", EventDirection::Assert)),
    ((0x2c << 24) | (0x05 << 16) | (0xff << 8) | 0xff, ("FRU_Deactivation_Requested", EventDirection::Assert)),
    ((0x2c << 24) | (0x06 << 16) | (0xff << 8) | 0xff, ("FRU_Deactivation_In_Progress", EventDirection::Assert)),
    ((0x2c << 24) | (0x07 << 16) | (0xff << 8) | 0xff, ("FRU_Communication_Lost", EventDirection::Deassert)),
    ])
  };
}
