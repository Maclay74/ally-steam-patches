use zbus::export::futures_util::StreamExt;
use zbus::{proxy, Connection, Result};

#[proxy(
    default_service = "org.shadowblip.PowerStation",
    default_path = "/org/shadowblip/Performance/GPU/card1",
    interface = "org.shadowblip.GPU.Card.TDP"
)]
trait GPU {
    #[zbus(property)]
    fn power_profiles_avaialable(&self) -> Result<Vec<String>>; // typo in powerstation interface :c

    #[zbus(property, signal)]
    fn power_profile(&self) -> Result<String>;

    #[zbus(property)]
    fn set_power_profile(&self, new_profile: &str) -> Result<()>;

    #[zbus(property, signal)]
    fn t_d_p(&self) -> Result<f64>;

    #[zbus(property)]
    fn set_t_d_p(&self, value: f64) -> Result<()>;
}

pub struct PowerStation {
    proxy: GPUProxy<'static>,
}

impl PowerStation {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        let proxy = GPUProxy::new(&connection).await?;
        Ok(PowerStation { proxy })
    }

    pub async fn get_gpu_profiles(&self) -> Result<Vec<String>> {
        self.proxy.power_profiles_avaialable().await
    }

    pub async fn get_gpu_profile(&self) -> Result<String> {
        self.proxy.power_profile().await
    }

    pub async fn set_gpu_profile(&self, new_profile: &str) -> Result<()> {
        self.proxy.set_power_profile(new_profile).await
    }

    pub async fn get_tdp(&self) -> Result<u8> {
        let tdp = self.proxy.t_d_p().await?;
        Ok(tdp.round() as u8)
    }

    pub async fn set_tdp(&self, value: u8) -> Result<()> {
        self.proxy.set_t_d_p(value as f64).await
    }

    pub async fn listen_gpu_profile(&self) {
        let mut profile_changed = self.proxy.receive_power_profile_changed().await;

        while let Some(_signal) = profile_changed.next().await {
            //let args = signal.get().await.unwrap();
            //println!("Power profile changed to `{:?}`", args);
            todo!();
        }
    }

    pub async fn listen_tdp(&self) {
        let mut tdp_changed = self.proxy.receive_t_d_p_changed().await;

        while let Some(_signal) = tdp_changed.next().await {
            //let args = signal.get().await.unwrap().round() as u8;
            //println!("TDP changed to `{:?}`", args);
            todo!();
        }
    }
}
