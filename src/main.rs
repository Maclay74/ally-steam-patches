use crate::powerstation::PowerStation;
use crate::steam::Steam;
use futures::join;
use std::sync::Arc;

mod patch;
mod powerstation;
mod steam;
mod web_server;

#[tokio::main]
async fn main() {
    let steam = Steam;

    let chunk_path = steam.get_chunk_file().unwrap();

    patch::patch(chunk_path).await;

    let power_station = PowerStation::new().await.unwrap();
    let power_station = Arc::new(power_station);

    join!(
        //power_station.listen_gpu_profile(),
        //power_station.listen_tdp(),
        web_server::web_server(Arc::clone(&power_station)),
    );
}
