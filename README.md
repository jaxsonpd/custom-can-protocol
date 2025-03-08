# custom-can-packet

A light-weight custom can bus like packet format for use in limited power applications

There are two implementions a rust a c.

The c implementation uses cmake and cab be included like
any other library. the packet processing module
for the c implementation is limited to a certain number
of identifiers which can be seen in custom_can_protocol/packet.c.


the rust can be included using cargo git or by linking a local
library.

## Protocol Definitions

The protocol is losely based on the can bus packet structure. With the following features:

- Optional acknowledgement (enforced by sending module not the protocol itself)
- CRC
- Resend last command
- Heartbeat at custom period

### Packet Structure

| Field | Length | Description |
| - | - | - |
| Start Byte `0x7E` | 1 Byte | Frame Delimiter |
| Identifier | 1 Byte | Command Identifier |
| Payload Length | 1 Byte | Length of the Payload |
| Payload | N Byte | The data |
| CRC | 2 Byte | CRC-16 for error checking |
| End Byte `0x7F` | 1 Byte | Frame Delimiter |

### Reserved Packet Identifiers

| Identifier | Function | Direction |
| - | - | - |
| 0x00 | Protocol Command | Both |

### Example Packet: Frequency Update

Command: Update frequencies
Data: Current frequency 123450 and standby frequency 124250

```yaml
Start Byte:      0x7E
Identifier:      0x01
Payload Length:  6
Payload:         [0x01, 0xE2, 0x40, 0x01, 0xE5, 0x02] (frequencies)
CRC:             0x1234 (not correct)
End Byte:        0x7E
```

### Protocol Command packet Structure

The protocol packet is used to send protocol commands the supported commands are:

| Name | Value | Description |
| - | - | - |
| Ack | 0x00 | Acknowledge the previous packet the second byte will be the identifier of the acknowledged command |
| Resend | 0xFE | Resend the last command |
| HeartBeat | 0xFF | A heart beat packet sent every second or user defined period.

```yaml
Start Byte:      0x7E
Identifier:      0x00
Payload Length:  2
Payload:         [0x00, 0x01] (Acknowledge 0x01 type packet)
CRC:             0x1234 (not correct)
End Byte:        0x7E
```

### CRC

Use the CR-16 Checksum with the 0x1021 polynomial.