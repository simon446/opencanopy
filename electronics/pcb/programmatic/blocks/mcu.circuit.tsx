// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="60mm" height="50mm" layers={2}>
  <capacitor name="C1" capacitance="22uF" footprint="0805" />
  <capacitor name="C2" capacitance="100nF" footprint="0402" />
  <capacitor name="C3" capacitance="100nF" footprint="0402" />
  <capacitor name="C_EN" capacitance="100nF" footprint="0402" />
  <chip name="J_DBG" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <chip name="J_USB" footprint="pinrow5" pinLabels={{pin1: "DP", pin2: "DN", pin3: "GND", pin4: "SHIELD", pin5: "VBUS"}} />
  <resistor name="R_EN" resistance="10k" footprint="0402" />
  <chip name="SW1" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="SW2" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="U1" footprint="pinrow26" pinLabels={{pin1: "3V3", pin2: "EN", pin3: "GND", pin4: "IO0", pin5: "IO10", pin6: "IO12", pin7: "IO13", pin8: "IO14", pin9: "IO15", pin10: "IO16", pin11: "IO17", pin12: "IO18", pin13: "IO19", pin14: "IO2", pin15: "IO20", pin16: "IO21", pin17: "IO4", pin18: "IO43", pin19: "IO44", pin20: "IO47", pin21: "IO48", pin22: "IO5", pin23: "IO6", pin24: "IO7", pin25: "IO8", pin26: "IO9"}} />

  <trace from="U1.3V3" to="net.P3V3" />
  <trace from="C1.pin1" to="net.P3V3" />
  <trace from="C2.pin1" to="net.P3V3" />
  <trace from="C3.pin1" to="net.P3V3" />
  <trace from="R_EN.pin1" to="net.P3V3" />
  <trace from="J_DBG.1" to="net.P3V3" />
  <trace from="U1.GND" to="net.GND" />
  <trace from="C1.pin2" to="net.GND" />
  <trace from="C2.pin2" to="net.GND" />
  <trace from="C3.pin2" to="net.GND" />
  <trace from="C_EN.pin2" to="net.GND" />
  <trace from="SW1.2" to="net.GND" />
  <trace from="SW2.2" to="net.GND" />
  <trace from="J_DBG.4" to="net.GND" />
  <trace from="J_USB.GND" to="net.GND" />
  <trace from="J_USB.SHIELD" to="net.GND" />
  <trace from="U1.IO8" to="net.SDA" />
  <trace from="U1.IO9" to="net.SCL" />
  <trace from="U1.EN" to="net.EN" />
  <trace from="R_EN.pin2" to="net.EN" />
  <trace from="C_EN.pin1" to="net.EN" />
  <trace from="SW2.1" to="net.EN" />
  <trace from="U1.IO0" to="net.IO0_BOOT" />
  <trace from="SW1.1" to="net.IO0_BOOT" />
  <trace from="U1.IO4" to="net.MOIST_ADC" />
  <trace from="U1.IO5" to="net.RES_LOW_SW" />
  <trace from="U1.IO6" to="net.RES_LEVEL_ADC" />
  <trace from="U1.IO7" to="net.LEAK_DET" />
  <trace from="U1.IO2" to="net.LED_NTC" />
  <trace from="U1.IO10" to="net.PUMP_PWM" />
  <trace from="U1.IO14" to="net.LED_DIM" />
  <trace from="U1.IO21" to="net.STATUS_DATA" />
  <trace from="U1.IO19" to="net.USB_DM" />
  <trace from="J_USB.DN" to="net.USB_DM" />
  <trace from="U1.IO20" to="net.USB_DP" />
  <trace from="J_USB.DP" to="net.USB_DP" />
  <trace from="U1.IO43" to="net.UART_TX" />
  <trace from="J_DBG.2" to="net.UART_TX" />
  <trace from="U1.IO44" to="net.UART_RX" />
  <trace from="J_DBG.3" to="net.UART_RX" />
  </board>
)
