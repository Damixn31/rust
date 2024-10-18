use std::{thread::sleep, time::Duration};

use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn capture_keys<F>(mut log_fn: F)
where
    F: FnMut(Keycode),
{
    let device_state = DeviceState::new();
    let mut key_press: Vec<Keycode> = Vec::new(); // para llevar un seguimiendo de las telcas
                                                  // presionadas

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        for key in &keys {
            if !key_press.contains(key) {
                log_fn(*key); // llamar a la funcion
                key_press.push(*key);
            }
        }
        key_press.retain(|key| keys.contains(key));
        sleep(Duration::from_millis(50));
    }
}
