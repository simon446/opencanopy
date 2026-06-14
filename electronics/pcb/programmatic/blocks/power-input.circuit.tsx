// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <capacitor name="C_BULK1" capacitance="100uF" footprint="1210" polarized />
  <capacitor name="C_BULK2" capacitance="10uF" footprint="1206" />
  <capacitor name="C_BULK3" capacitance="100nF" footprint="0402" />
  <diode name="D1" footprint="sod123" />
  <diode name="DZ_RP" footprint="sod123" />
  <fuse name="F1" currentRating="6.3A" voltageRating="32V" footprint="1206" />
  <chip name="J_PWR" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <mosfet name="Q2" channelType="p" mosfetMode="enhancement" footprint="sot23" />
  <resistor name="R_RP" resistance="100k" footprint="0402" />

  <trace from="J_PWR.1" to="net.P24V_IN" />
  <trace from="F1.pin1" to="net.P24V_IN" />
  <trace from="F1.pin2" to="net.P24V_FUSED" />
  <trace from="Q2.source" to="net.P24V_FUSED" />
  <trace from="DZ_RP.cathode" to="net.P24V_FUSED" />
  <trace from="Q2.gate" to="net.RP_GATE" />
  <trace from="DZ_RP.anode" to="net.RP_GATE" />
  <trace from="R_RP.pin1" to="net.RP_GATE" />
  <trace from="Q2.drain" to="net.P24V" />
  <trace from="D1.cathode" to="net.P24V" />
  <trace from="C_BULK1.pin1" to="net.P24V" />
  <trace from="C_BULK2.pin1" to="net.P24V" />
  <trace from="C_BULK3.pin1" to="net.P24V" />
  <trace from="J_PWR.2" to="net.GND" />
  <trace from="D1.anode" to="net.GND" />
  <trace from="C_BULK1.pin2" to="net.GND" />
  <trace from="C_BULK2.pin2" to="net.GND" />
  <trace from="C_BULK3.pin2" to="net.GND" />
  <trace from="R_RP.pin2" to="net.GND" />
  </board>
)
