# STMicro L9961 Industrial Battery Management System Driver

This driver aims to provide a robust API for building battery packs utilizing the [STMicro L9961 BMS Chip](https://www.st.com/en/power-management/l9961.html).
It supports a blocking as well as asynchronous API.
The crate is built on top of the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)  traits and is `no_std` compatible.

## Driver Design

The driver is built in two layers, the low-level register access layer and the high-level API layer.
Conversion logic for register codes is implemented in the conversions module.
The high level API uses standard units and converts to chip specific code units automatically.

## Register Information

The L9961 BMS chip has 48 registers, each with a unique address and definition.
The [register map](./register_info.md) provides the address, name, and layout of each register.
