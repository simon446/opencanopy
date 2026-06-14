# Firmware API reference

Auto-generated Rust API documentation for the **host-testable firmware crates**, built from source
with `cargo doc` and published alongside this site under `/api`.

<ul>
  <li><a href="api/control/index.html"><strong><code>control</code> crate API</strong></a>
      — the platform-agnostic <code>no_std</code> control logic: state machine, irrigation, light,
      climate, calibration, LED status, the I2C device protocol, and the <code>hal.rs</code> traits.</li>
  <li><a href="api/sim/index.html"><strong><code>sim</code> crate API</strong></a>
      — the host plant/environment simulator that drives the real <code>control</code> crate.</li>
</ul>

These are regenerated whenever `firmware/control/**` or `firmware/sim/**` changes (see the
[docs workflow](https://github.com/simon446/opencanopy/blob/main/.github/workflows/docs.yml)).

> The on-target `controller` crate is esp-hal / Xtensa-only and is **not** part of this host-doc
> build (it needs the Espressif toolchain to compile). Its structure is documented in
> [`firmware/controller/README.md`](https://github.com/simon446/opencanopy/blob/main/firmware/controller/README.md).
