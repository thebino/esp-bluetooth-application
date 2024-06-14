#![no_std]
#![no_main]

use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    attribute_server::AttributeServer,
    gatt, Ble, HciConnector,
};
use esp_backtrace as _;
use esp_hal::rng::Rng;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl};
use esp_wifi::ble::controller::BleConnector;
use esp_wifi::EspWifiInitFor;

use crate::rng::EspRng;

mod rng;

pub const SOC_NAME: &str = "ESP32-S3";

#[entry]
fn main() -> ! {
    log::info!("starting ble app: setup");

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();

    let rng = Rng::new(peripherals.RNG);
    let timer = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1, &clocks, None).timer0;
    let init = esp_wifi::initialize(
        EspWifiInitFor::Ble,
        timer,
        rng,
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    let mut bluetooth = peripherals.BT;

    loop {
        log::info!("Loop started");
        let connector = BleConnector::new(&init, &mut bluetooth);
        let hci = HciConnector::new(connector, esp_wifi::current_millis);
        let mut ble = Ble::new(&hci);

        log::info!("{:?}", ble.init());
        log::info!("{:?}", ble.cmd_set_le_advertising_parameters());
        log::info!(
            "{:?}",
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
                    AdStructure::CompleteLocalName(SOC_NAME),
                ])
                .unwrap()
            )
        );

        log::info!("{:?}", ble.cmd_set_le_advertise_enable(true));

        log::info!("started advertising");

        let mut rf = |_offset: usize, data: &mut [u8]| {
            data[..20].copy_from_slice(&b"Hello Bare-Metal BLE"[..]);
            17
        };
        let mut wf = |offset: usize, data: &[u8]| {
            log::info!("RECEIVED: {} {:?}", offset, data);
        };

        let mut wf2 = |offset: usize, data: &[u8]| {
            log::info!("RECEIVED: {} {:?}", offset, data);
        };

        let mut rf3 = |_offset: usize, data: &mut [u8]| {
            data[..5].copy_from_slice(&b"Hola!"[..]);
            5
        };
        let mut wf3 = |offset: usize, data: &[u8]| {
            log::info!("RECEIVED: Offset {}, data {:?}", offset, data);
        };

        // creates gatt_attributes
        gatt!([service {
            uuid: "00000020-1000-2000-3000-111122223333",
            characteristics: [
                characteristic {
                    uuid: "00000021-1000-2000-3000-111122223333",
                    read: rf,
                    write: wf,
                },
                characteristic {
                    uuid: "00000022-1000-2000-3000-111122223333",
                    write: wf2,
                },
                characteristic {
                    name: "test_characteristic",
                    uuid: "00000023-1000-2000-3000-111122223333",
                    notify: true,
                    read: rf3,
                    write: wf3,
                },
            ],
        },]);

        let mut ble_rng = EspRng(rng);
        let _srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut ble_rng);

        // loop {
        //     let mut notification = None;
        //
        // if button.is_low().unwrap() && debounce_cnt > 0 {
        //     debounce_cnt -= 1;
        //     if debounce_cnt == 0 {
        //         let mut cccd = [0u8; 1];
        //         if let Some(1) = srv.get_characteristic_value(
        //             my_characteristic_notify_enable_handle,
        //             0,
        //             &mut cccd,
        //         ) {
        //             // if notifications enabled
        //             if cccd[0] == 1 {
        //                 notification = Some(NotificationData::new(
        //                     my_characteristic_handle,
        //                     &b"Notification"[..],
        //                 ));
        //             }
        //         }
        //     }
        // };
        //
        // if button.is_high().unwrap() {
        //     debounce_cnt = 500;
        // }

        // match srv.do_work_with_notification(notification) {
        //     Ok(res) => {
        //         if let WorkResult::GotDisconnected = res {
        //             break;
        //         }
        //     }
        //     Err(err) => {
        //         log::info!("{:?}", err);
        //     }
        // }
        // }
    }
}
