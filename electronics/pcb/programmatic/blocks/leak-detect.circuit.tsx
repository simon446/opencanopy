// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <capacitor name="C_LK" capacitance="100nF" footprint="0402" />
  <capacitor name="C_U5" capacitance="100nF" footprint="0402" />
  <chip name="J_LEAK" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <resistor name="R_LK_OUT" resistance="10k" footprint="0402" />
  <resistor name="R_LK_PU" resistance="100k" footprint="0402" />
  <resistor name="R_LK_RA" resistance="10k" footprint="0402" />
  <resistor name="R_LK_RB" resistance="10k" footprint="0402" />
  <chip name="U5" footprint="pinrow7" pinLabels={{pin1: "GND", pin2: "IN1P", pin3: "IN1N", pin4: "IN2P", pin5: "IN2N", pin6: "OUT1", pin7: "VCC"}} />

  <trace from="U5.VCC" to="net.P3V3" />
  <trace from="C_U5.pin1" to="net.P3V3" />
  <trace from="R_LK_PU.pin1" to="net.P3V3" />
  <trace from="R_LK_RA.pin1" to="net.P3V3" />
  <trace from="R_LK_OUT.pin1" to="net.P3V3" />
  <trace from="U5.GND" to="net.GND" />
  <trace from="C_U5.pin2" to="net.GND" />
  <trace from="R_LK_RB.pin2" to="net.GND" />
  <trace from="C_LK.pin2" to="net.GND" />
  <trace from="U5.IN2P" to="net.GND" />
  <trace from="U5.IN2N" to="net.GND" />
  <trace from="J_LEAK.2" to="net.GND" />
  <trace from="J_LEAK.1" to="net.LEAK_SENSE" />
  <trace from="R_LK_PU.pin2" to="net.LEAK_SENSE" />
  <trace from="U5.IN1P" to="net.LEAK_SENSE" />
  <trace from="C_LK.pin1" to="net.LEAK_SENSE" />
  <trace from="U5.IN1N" to="net.LEAK_REF" />
  <trace from="R_LK_RA.pin2" to="net.LEAK_REF" />
  <trace from="R_LK_RB.pin1" to="net.LEAK_REF" />
  <trace from="U5.OUT1" to="net.LEAK_DET" />
  <trace from="R_LK_OUT.pin2" to="net.LEAK_DET" />
  </board>
)
