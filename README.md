# pzem-cli

A command-line interface for the PZEM004T energy monitor as a front-end for the [embedded-hal rust driver](https://github.com/iostapyshyn/pzem004t).

## Usage
```console
foo@bar:~$ pzem-cli [OPTIONS] <PORT> <SUBCOMMAND>
```
### Options
|                  Option |                                                 Description |
|-------------------------|-------------------------------------------------------------|
|       -a, --addr <addr> | Slave address of the sensor in hexadecimal format (e.g. 5a) |
| -t, --timeout <timeout> |                  Response awaiting timeout in milliseconds. |

The address used by default is the universal address for single-slave environments: `0xf8`. Every slave is guaranteed to respond to that address.
The default timeout is `500` milliseconds.

### Commands
|         Command |                                                                             Description |
|-----------------|-----------------------------------------------------------------------------------------|
| read            | Read and display the measurement results.                                               |
| reset           | Reset the internal energy counter to 0.                                                 |
| addr [VAL]      | Set the MODBUS-RTU address to **VAL** if given, otherwise display the current address.  |
| threshold [VAL] | Set the alarm threshold to **VAL** if given, otherwise display the current value.       |

## Example
```console
foo@bar:~$ pzem-cli -t 1000 /dev/ttyACM0 addr bf
foo@bar:~$ pzem-cli -a bf /dev/ttyACM0 read
Voltage: 241.6 V
Current: 0.000 A
Power: 0.0 W
Energy: 0.000 kWh
Frequency: 50.0 Hz
Power factor: 0.00
Alarm: off
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in hexi by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
