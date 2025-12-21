Install probe-rs as specified [here](https://probe.rs/docs/getting-started/installation/).

Grove moisture sensor plugged in at 3.3V, GND, A0

Build and flash:
- `cargo flash --release --chip R7FA4M1AB --target thumbv7em-none-eabihf`

Debug:
- build and flash without --release flag
- launch vscode probe-rs debugger with config from .vscode/

Serial:
- ![img.png](documentation/pin-functions.png)
- ![serial-flow.png](documentation/serial-flow.png) pg. 751
