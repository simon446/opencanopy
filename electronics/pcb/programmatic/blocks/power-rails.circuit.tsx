// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <capacitor name="C_BT5" capacitance="10nF" footprint="0402" />
  <capacitor name="C_IN33" capacitance="1uF" footprint="0402" />
  <capacitor name="C_IN5" capacitance="10uF" footprint="1206" />
  <capacitor name="C_OUT33" capacitance="1uF" footprint="0402" />
  <capacitor name="C_OUT5" capacitance="22uF" footprint="1210" />
  <diode name="D_CAT5" footprint="sod123" />
  <inductor name="L5" inductance="10uH" footprint="1210" />
  <resistor name="R_FB5A" resistance="22k" footprint="0402" />
  <resistor name="R_FB5B" resistance="7.15k" footprint="0402" />
  <chip name="U7" footprint="pinrow6" pinLabels={{pin1: "BOOT", pin2: "ENA", pin3: "GND", pin4: "PH", pin5: "VIN", pin6: "VSENSE"}} />
  <chip name="U8" footprint="pinrow4" pinLabels={{pin1: "EN", pin2: "GND", pin3: "VIN", pin4: "VOUT"}} />

  <trace from="U7.VIN" to="net.P24V" />
  <trace from="U7.ENA" to="net.P24V" />
  <trace from="C_IN5.pin1" to="net.P24V" />
  <trace from="U7.PH" to="net.PH5" />
  <trace from="L5.pin1" to="net.PH5" />
  <trace from="D_CAT5.cathode" to="net.PH5" />
  <trace from="C_BT5.pin2" to="net.PH5" />
  <trace from="L5.pin2" to="net.P5V" />
  <trace from="C_OUT5.pin1" to="net.P5V" />
  <trace from="R_FB5A.pin1" to="net.P5V" />
  <trace from="U8.VIN" to="net.P5V" />
  <trace from="U8.EN" to="net.P5V" />
  <trace from="C_IN33.pin1" to="net.P5V" />
  <trace from="U7.VSENSE" to="net.FB5" />
  <trace from="R_FB5A.pin2" to="net.FB5" />
  <trace from="R_FB5B.pin1" to="net.FB5" />
  <trace from="U7.BOOT" to="net.BOOT5" />
  <trace from="C_BT5.pin1" to="net.BOOT5" />
  <trace from="U8.VOUT" to="net.P3V3" />
  <trace from="C_OUT33.pin1" to="net.P3V3" />
  <trace from="U7.GND" to="net.GND" />
  <trace from="C_IN5.pin2" to="net.GND" />
  <trace from="C_OUT5.pin2" to="net.GND" />
  <trace from="R_FB5B.pin2" to="net.GND" />
  <trace from="D_CAT5.anode" to="net.GND" />
  <trace from="U8.GND" to="net.GND" />
  <trace from="C_IN33.pin2" to="net.GND" />
  <trace from="C_OUT33.pin2" to="net.GND" />
  </board>
)
