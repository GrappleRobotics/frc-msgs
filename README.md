# FRC Messages
**Grapple's repository of FRC CAN and other protocols**

This repository contains all of the messages we use in Grapple for communicating with our products, as well as infrastructure for CAN message fragmentation and defragmentation.

Messages are declaratively created using [binmarshal](https://github.com/GrappleRobotics/binmarshal), which abstracts the low-level transport of the messages so you can focus on making products work.

## Examples
```rust
let my_msg = Message::new(
  device_id,
  ManufacturerMessage::Grapple(GrappleDeviceMessage::DistanceSensor(
    LaserCanMessage::Status(LaserCanStatusFrame {
      // ...
    })
  ))
);

let mut writer = binmarshal::VecBitWriter::new();
my_msg.write(&mut writer, ());
let slice: &[u8] = writer.slice();

// ...
```