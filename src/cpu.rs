use sysinfo::Components;

pub fn show_components() {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        let id = component.id().unwrap_or("...");
        let lable = component.label();
        let temperature = component.temperature().unwrap_or(0.0);

        tracing::info!("{id} - {lable} - {temperature}");
    }
}
