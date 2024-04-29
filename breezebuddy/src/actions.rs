use std::collections::HashMap;
use sysinfo::Components;

/// used to truncate labels
fn truncate(string: &str, l: usize) -> String {
    if string.len() > l {
        string.chars().take(l).collect()
    } else {
        string.to_string()
    }
}

/// returns a hashmap of label/temperature pairs
pub fn get_sensors() -> HashMap<String, f32> {
    let components = Components::new_with_refreshed_list();
    let mut sensor_data = HashMap::new();

    for component in &components {
        let label = truncate(component.label(), 10);
        let temperature = component.temperature();

        // Check if temperature is not 0 and not NaN
        if temperature != 0.0 && !temperature.is_nan() {
            println!("{} -> {}", label, temperature);
            sensor_data.insert(label, temperature);
        }
    }
    return sensor_data;
}
