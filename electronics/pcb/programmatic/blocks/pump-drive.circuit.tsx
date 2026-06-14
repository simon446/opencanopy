// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <diode name="D2" footprint="sod123" />
  <chip name="J_PUMP" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <mosfet name="Q1" channelType="n" mosfetMode="enhancement" footprint="sot23" />
  <resistor name="R1" resistance="10k" footprint="0402" />
  <resistor name="R2" resistance="100" footprint="0402" />

  <trace from="J_PUMP.1" to="net.PUMP_P" />
  <trace from="D2.cathode" to="net.PUMP_P" />
  <trace from="J_PUMP.2" to="net.PUMP_RET" />
  <trace from="Q1.drain" to="net.PUMP_RET" />
  <trace from="D2.anode" to="net.PUMP_RET" />
  <trace from="Q1.gate" to="net.PUMP_GATE" />
  <trace from="R2.pin2" to="net.PUMP_GATE" />
  <trace from="R1.pin1" to="net.PUMP_GATE" />
  <trace from="Q1.source" to="net.GND" />
  <trace from="R1.pin2" to="net.GND" />
  <trace from="R2.pin1" to="net.PUMP_PWM" />
  </board>
)
