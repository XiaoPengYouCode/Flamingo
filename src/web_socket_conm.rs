#[derive(Clone, PartialEq, Debug)]
pub struct WebSocketDevice {
    ip: String,
    mac: String,
}

impl WebSocketDevice {
    pub fn new(ip: String, mac: String) -> Self {
        Self { ip, mac }
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub fn get_mac(&self) -> String {
        self.mac.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WebSocketData {
    data_heart_rate: f32,
    data_oxygen_saturation: f32,
    time_stamp: String,
}

impl WebSocketData {
    pub fn new(data: f32, data2: f32, time: String) -> Self {
        Self {
            data_heart_rate: data,
            data_oxygen_saturation: data2,
            time_stamp: time,
        }
    }

    pub fn get_data(&self) -> f32 {
        self.data_heart_rate
    }
}
