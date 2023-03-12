//! BLE SoftDevice config and tasks
use core::mem;
use embassy_nrf::interrupt::Priority;
use nrf_softdevice::{raw, Softdevice};

/// Returns SoftDevice configuration
pub fn softdevice_config() -> nrf_softdevice::Config {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;

    nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 4,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 2,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256}),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT.into(),
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: raw::BLE_GAP_ADV_SET_COUNT_DEFAULT as u8,
            //periph_role_count: raw::BLE_GAP_ROLE_COUNT_PERIPH_DEFAULT as u8,
            periph_role_count: 2u8,
            central_role_count: 0,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0)
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"BlueTest" as *const u8 as _,
            current_len: 8,
            max_len: 8,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    }

}

/// SoftDevice task, just let it run.
#[embassy_executor::task]
pub async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}