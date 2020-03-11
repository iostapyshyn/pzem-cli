# pzem-cli

A command-line interface for the PZEM004T energy monitor as a demo of the embedded-hal rust driver.

## Usage
```console
foo@bar:~$ pzem-cli [OPTIONS] <PORT> <SUBCOMMAND>
```
### Options
|                  Option |                                                 Description |
|-------------------------|-------------------------------------------------------------|
|       -a, --addr <addr> | Slave address of the sensor in hexadecimal format (e.g. 5a) |
| -t, --timeout <timeout> |                  Response awaiting timeout in milliseconds. |

The address used by default is the universal address for single-slave environments: 0xF8. Every slave is guaranteed to respond to that address.
The default timeout is 500 milliseconds.

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
```
