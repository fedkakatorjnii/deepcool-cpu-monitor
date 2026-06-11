use sysinfo::Components;

pub fn get_temperature(name: &str) -> f32 {
    let components = Components::new_with_refreshed_list();

    for component in &components {
        if name == component.label()
            && let Some(temperature) = component.temperature()
        {
            return temperature;
        }
    }

    0.0
}
