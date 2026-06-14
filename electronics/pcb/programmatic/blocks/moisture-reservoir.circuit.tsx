// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <capacitor name="C_MOIST" capacitance="10nF" footprint="0402" />
  <chip name="J_MOIST" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <chip name="J_RES" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <resistor name="R_MOIST" resistance="1k" footprint="0402" />

  <trace from="J_MOIST.1" to="net.P3V3" />
  <trace from="C_MOIST.pin2" to="net.GND" />
  <trace from="J_MOIST.2" to="net.GND" />
  <trace from="J_RES.2" to="net.GND" />
  <trace from="J_MOIST.3" to="net.MOIST_SIG" />
  <trace from="R_MOIST.pin1" to="net.MOIST_SIG" />
  <trace from="R_MOIST.pin2" to="net.MOIST_ADC" />
  <trace from="C_MOIST.pin1" to="net.MOIST_ADC" />
  <trace from="J_RES.1" to="net.RES_LOW_SW" />
  <trace from="J_RES.3" to="net.RES_LEVEL_ADC" />
  </board>
)
