# deepcool-cpu-monitor

Software for monitor DEEPCOOL AK400 DIGITAL.

## Building

1. Build release

```bash
cargo build --release
```

2. Move release to some custom path.

```bash
cp ./target/release/deepcool-cpu-monitor /path/to/bin
```

## Configuration

You neet to create config.toml file:

- vendor_id - ;
- product_id - ;
- cpu - cpu lable;
- time - update interval (`milliseconds`);
- log_level - `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`;

```bash
lsusb -v | rg "DeepCool"
lsusb -v | rg "idVendor"
lsusb -v | rg "idProduct"
```

You can use `lsusb` or `lsusb -v` for find `vendor_id` and `product_id`.
Example use `lsusb`:
You can use `3633:0001` -> `0x3633:0x0001`

```bash
...
xxxxxxxxxxxxxxxxxx: ID 3633:0001 DeepCool AK400-DIGITAL
...
```

You can use `./target/release/deepcool-cpu-monitor -s` for shoe component list.
P.S. I don't know the correct way to identify the CPU to display its temperature.

Example config file (`example.config.toml`):

```toml
vendor_id = 0x3633
product_id = 0x0001
cpu = "k10temp Tctl"
time = 2000
log_level = "INFO"
```

## Running

You neet root permission for running.

1. Create config file:

2. Run with config file:

```bash
./target/release/deepcool-cpu-monitor -c /home/user/.config/deepcool/config.toml
```
