// data.rs

pub struct Data {
    pub id: Option<String>,
    pub time: Vec<f64>,
    pub obs: Vec<f64>,
    pub dose: f64,
}

impl Data {
    // Constructor for the Data struct
    pub fn new(id: Option<String>, time: Vec<f64>, obs: Vec<f64>, dose: f64) -> Data {
        Data {
            id,
            time,
            obs,
            dose,
        }
    }

    // Method to calculate AUC using the Trapezoidal rule
    pub fn calculate_auc(&self) -> Result<f64, String> {
        let mut auc = 0.0;

        if self.time.len() != self.obs.len() {
            return Err("Time and observation vectors have different lengths.".to_string());
        } else if self.time.is_empty() {
            return Err("Time vector is empty.".to_string());
        }

        // Apply the Trapezoidal rule for AUC calculation
        for i in 0..self.time.len() - 1 {
            let dt = self.time[i + 1] - self.time[i];
            let avg_obs = (self.obs[i] + self.obs[i + 1]) / 2.0;
            auc += dt * avg_obs;
        }

        Ok(auc)
    }
}
