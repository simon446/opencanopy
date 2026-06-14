// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <chip name="J_LED" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <chip name="J_STATUS" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <resistor name="R_DATA" resistance="330" footprint="0402" />
  <resistor name="R_DIM" resistance="100" footprint="0402" />

  <trace from="J_LED.1" to="net.P24V" />
  <trace from="J_STATUS.1" to="net.P5V" />
  <trace from="J_LED.2" to="net.GND" />
  <trace from="J_STATUS.2" to="net.GND" />
  <trace from="J_LED.4" to="net.LED_NTC" />
  <trace from="R_DIM.pin1" to="net.LED_DIM" />
  <trace from="R_DIM.pin2" to="net.LED_DIM_OUT" />
  <trace from="J_LED.3" to="net.LED_DIM_OUT" />
  <trace from="R_DATA.pin1" to="net.STATUS_DATA" />
  <trace from="R_DATA.pin2" to="net.STATUS_DATA_OUT" />
  <trace from="J_STATUS.3" to="net.STATUS_DATA_OUT" />
  </board>
)
