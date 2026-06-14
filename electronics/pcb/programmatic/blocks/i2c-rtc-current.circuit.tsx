// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <chip name="BT1" footprint="pinrow2" pinLabels={{pin1: "P", pin2: "N"}} />
  <capacitor name="C_INA" capacitance="100nF" footprint="0402" />
  <capacitor name="C_RTC" capacitance="100nF" footprint="0402" />
  <chip name="J_SENS" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <resistor name="R_SCL" resistance="4.7k" footprint="0402" />
  <resistor name="R_SDA" resistance="4.7k" footprint="0402" />
  <resistor name="R_SHUNT" resistance="0.1" footprint="2512" />
  <chip name="U3" footprint="pinrow5" pinLabels={{pin1: "GND", pin2: "SCL", pin3: "SDA", pin4: "VBAT", pin5: "VCC"}} />
  <chip name="U4" footprint="pinrow8" pinLabels={{pin1: "A0", pin2: "A1", pin3: "GND", pin4: "INP", pin5: "INN", pin6: "SCL", pin7: "SDA", pin8: "VS"}} />

  <trace from="R_SHUNT.pin1" to="net.P24V" />
  <trace from="U4.INP" to="net.P24V" />
  <trace from="R_SHUNT.pin2" to="net.PUMP_P" />
  <trace from="U4.INN" to="net.PUMP_P" />
  <trace from="R_SDA.pin1" to="net.P3V3" />
  <trace from="R_SCL.pin1" to="net.P3V3" />
  <trace from="U3.VCC" to="net.P3V3" />
  <trace from="C_RTC.pin1" to="net.P3V3" />
  <trace from="U4.VS" to="net.P3V3" />
  <trace from="C_INA.pin1" to="net.P3V3" />
  <trace from="J_SENS.1" to="net.P3V3" />
  <trace from="U3.GND" to="net.GND" />
  <trace from="C_RTC.pin2" to="net.GND" />
  <trace from="BT1.N" to="net.GND" />
  <trace from="U4.GND" to="net.GND" />
  <trace from="C_INA.pin2" to="net.GND" />
  <trace from="U4.A0" to="net.GND" />
  <trace from="U4.A1" to="net.GND" />
  <trace from="J_SENS.2" to="net.GND" />
  <trace from="R_SDA.pin2" to="net.SDA" />
  <trace from="U3.SDA" to="net.SDA" />
  <trace from="U4.SDA" to="net.SDA" />
  <trace from="J_SENS.3" to="net.SDA" />
  <trace from="R_SCL.pin2" to="net.SCL" />
  <trace from="U3.SCL" to="net.SCL" />
  <trace from="U4.SCL" to="net.SCL" />
  <trace from="J_SENS.4" to="net.SCL" />
  <trace from="U3.VBAT" to="net.VBAT" />
  <trace from="BT1.P" to="net.VBAT" />
  </board>
)
