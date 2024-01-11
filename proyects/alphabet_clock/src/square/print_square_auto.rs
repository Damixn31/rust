use std::sync::Arc;
use std::time::Duration;

use mutex::Mutex;
use tokio::time::sleep;

use crate::get_position_highlighted;
use crate::print_square;

pub async fn print_square_auto(position_highlighted: Arc<Mutex<Vec<(usize, usize)>>>) {
    loop {
        let hour_current = chrono::Local::now().format("%H:%M:%S").to_string();

        //let mut position_highlighted_guard = position_highlighted.lock().unwrap();
        let position_highlighted_update = get_position_highlighted(&hour_current);

        //*position_highlighted_guard = position_highlighted_update.clone();

        //drop(position_highlighted_guard);

        print_square(&position_highlighted_update, &hour_current);

        sleep(Duration::from_secs(5)).await;
    }
}
