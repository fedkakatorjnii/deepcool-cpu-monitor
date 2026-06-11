use std::thread::sleep;

use sysinfo::{CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, RefreshKind, System};

pub fn get_usage() -> f32 {
    let mut sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

    sys.refresh_cpu_usage();

    sleep(MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();

    let mut data = 0.0;
    let mut count = 0;

    for cpu in sys.cpus().iter() {
        let cpu_usage = cpu.cpu_usage();

        count += 1;
        data += cpu_usage;
    }

    if count == 0 || data == 0.0 {
        return 0.0;
    }

    data / count as f32
}
