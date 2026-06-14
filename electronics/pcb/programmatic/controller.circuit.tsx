// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="110mm" height="75mm" layers={2}>
  <chip name="BT1" footprint="pinrow2" pinLabels={{pin1: "P", pin2: "N"}} />
  <capacitor name="C1" capacitance="22uF" footprint="0805" />
  <capacitor name="C2" capacitance="100nF" footprint="0402" />
  <capacitor name="C3" capacitance="100nF" footprint="0402" />
  <capacitor name="C_BT5" capacitance="10nF" footprint="0402" />
  <capacitor name="C_BULK1" capacitance="100uF" footprint="1210" polarized />
  <capacitor name="C_BULK2" capacitance="10uF" footprint="1206" />
  <capacitor name="C_BULK3" capacitance="100nF" footprint="0402" />
  <capacitor name="C_EN" capacitance="100nF" footprint="0402" />
  <capacitor name="C_IN33" capacitance="1uF" footprint="0402" />
  <capacitor name="C_IN5" capacitance="10uF" footprint="1206" />
  <capacitor name="C_INA" capacitance="100nF" footprint="0402" />
  <capacitor name="C_LK" capacitance="100nF" footprint="0402" />
  <capacitor name="C_MOIST" capacitance="10nF" footprint="0402" />
  <capacitor name="C_OUT33" capacitance="1uF" footprint="0402" />
  <capacitor name="C_OUT5" capacitance="22uF" footprint="1210" />
  <capacitor name="C_RTC" capacitance="100nF" footprint="0402" />
  <capacitor name="C_U5" capacitance="100nF" footprint="0402" />
  <diode name="D1" footprint="sod123" />
  <diode name="D2" footprint="sod123" />
  <diode name="DZ_RP" footprint="sod123" />
  <diode name="D_CAT5" footprint="sod123" />
  <fuse name="F1" currentRating="6.3A" voltageRating="32V" footprint="1206" />
  <chip name="J_DBG" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <chip name="J_LEAK" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="J_LED" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <chip name="J_MOIST" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <chip name="J_PUMP" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="J_PWR" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="J_RES" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <chip name="J_SENS" footprint="pinrow4" pinLabels={{pin1: "1", pin2: "2", pin3: "3", pin4: "4"}} />
  <chip name="J_STATUS" footprint="pinrow3" pinLabels={{pin1: "1", pin2: "2", pin3: "3"}} />
  <chip name="J_USB" footprint="pinrow5" pinLabels={{pin1: "DP", pin2: "DN", pin3: "GND", pin4: "SHIELD", pin5: "VBUS"}} />
  <inductor name="L5" inductance="10uH" footprint="1210" />
  <mosfet name="Q1" channelType="n" mosfetMode="enhancement" footprint="sot23" />
  <mosfet name="Q2" channelType="p" mosfetMode="enhancement" footprint="sot23" />
  <resistor name="R1" resistance="10k" footprint="0402" />
  <resistor name="R2" resistance="100" footprint="0402" />
  <resistor name="R_DATA" resistance="330" footprint="0402" />
  <resistor name="R_DIM" resistance="100" footprint="0402" />
  <resistor name="R_EN" resistance="10k" footprint="0402" />
  <resistor name="R_FB5A" resistance="22k" footprint="0402" />
  <resistor name="R_FB5B" resistance="7.15k" footprint="0402" />
  <resistor name="R_LK_OUT" resistance="10k" footprint="0402" />
  <resistor name="R_LK_PU" resistance="100k" footprint="0402" />
  <resistor name="R_LK_RA" resistance="10k" footprint="0402" />
  <resistor name="R_LK_RB" resistance="10k" footprint="0402" />
  <resistor name="R_MOIST" resistance="1k" footprint="0402" />
  <resistor name="R_RP" resistance="100k" footprint="0402" />
  <resistor name="R_SCL" resistance="4.7k" footprint="0402" />
  <resistor name="R_SDA" resistance="4.7k" footprint="0402" />
  <resistor name="R_SHUNT" resistance="0.1" footprint="2512" />
  <chip name="SW1" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="SW2" footprint="pinrow2" pinLabels={{pin1: "1", pin2: "2"}} />
  <chip name="U1" footprint="pinrow26" pinLabels={{pin1: "3V3", pin2: "EN", pin3: "GND", pin4: "IO0", pin5: "IO10", pin6: "IO12", pin7: "IO13", pin8: "IO14", pin9: "IO15", pin10: "IO16", pin11: "IO17", pin12: "IO18", pin13: "IO19", pin14: "IO2", pin15: "IO20", pin16: "IO21", pin17: "IO4", pin18: "IO43", pin19: "IO44", pin20: "IO47", pin21: "IO48", pin22: "IO5", pin23: "IO6", pin24: "IO7", pin25: "IO8", pin26: "IO9"}} />
  <chip name="U3" footprint="pinrow5" pinLabels={{pin1: "GND", pin2: "SCL", pin3: "SDA", pin4: "VBAT", pin5: "VCC"}} />
  <chip name="U4" footprint="pinrow8" pinLabels={{pin1: "A0", pin2: "A1", pin3: "GND", pin4: "INP", pin5: "INN", pin6: "SCL", pin7: "SDA", pin8: "VS"}} />
  <chip name="U5" footprint="pinrow7" pinLabels={{pin1: "GND", pin2: "IN1P", pin3: "IN1N", pin4: "IN2P", pin5: "IN2N", pin6: "OUT1", pin7: "VCC"}} />
  <chip name="U7" footprint="pinrow6" pinLabels={{pin1: "BOOT", pin2: "ENA", pin3: "GND", pin4: "PH", pin5: "VIN", pin6: "VSENSE"}} />
  <chip name="U8" footprint="pinrow4" pinLabels={{pin1: "EN", pin2: "GND", pin3: "VIN", pin4: "VOUT"}} />

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
  <trace from="U7.VIN" to="net.P24V" />
  <trace from="U7.ENA" to="net.P24V" />
  <trace from="C_IN5.pin1" to="net.P24V" />
  <trace from="J_LED.1" to="net.P24V" />
  <trace from="R_SHUNT.pin1" to="net.P24V" />
  <trace from="U4.INP" to="net.P24V" />
  <trace from="R_SHUNT.pin2" to="net.PUMP_P" />
  <trace from="U4.INN" to="net.PUMP_P" />
  <trace from="J_PUMP.1" to="net.PUMP_P" />
  <trace from="D2.cathode" to="net.PUMP_P" />
  <trace from="J_PUMP.2" to="net.PUMP_RET" />
  <trace from="Q1.drain" to="net.PUMP_RET" />
  <trace from="D2.anode" to="net.PUMP_RET" />
  <trace from="Q1.gate" to="net.PUMP_GATE" />
  <trace from="R2.pin2" to="net.PUMP_GATE" />
  <trace from="R1.pin1" to="net.PUMP_GATE" />
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
  <trace from="J_STATUS.1" to="net.P5V" />
  <trace from="U7.VSENSE" to="net.FB5" />
  <trace from="R_FB5A.pin2" to="net.FB5" />
  <trace from="R_FB5B.pin1" to="net.FB5" />
  <trace from="U7.BOOT" to="net.BOOT5" />
  <trace from="C_BT5.pin1" to="net.BOOT5" />
  <trace from="U8.VOUT" to="net.P3V3" />
  <trace from="C_OUT33.pin1" to="net.P3V3" />
  <trace from="U1.3V3" to="net.P3V3" />
  <trace from="C1.pin1" to="net.P3V3" />
  <trace from="C2.pin1" to="net.P3V3" />
  <trace from="C3.pin1" to="net.P3V3" />
  <trace from="R_EN.pin1" to="net.P3V3" />
  <trace from="R_SDA.pin1" to="net.P3V3" />
  <trace from="R_SCL.pin1" to="net.P3V3" />
  <trace from="U3.VCC" to="net.P3V3" />
  <trace from="C_RTC.pin1" to="net.P3V3" />
  <trace from="U4.VS" to="net.P3V3" />
  <trace from="C_INA.pin1" to="net.P3V3" />
  <trace from="U5.VCC" to="net.P3V3" />
  <trace from="C_U5.pin1" to="net.P3V3" />
  <trace from="R_LK_PU.pin1" to="net.P3V3" />
  <trace from="R_LK_RA.pin1" to="net.P3V3" />
  <trace from="R_LK_OUT.pin1" to="net.P3V3" />
  <trace from="J_MOIST.1" to="net.P3V3" />
  <trace from="J_SENS.1" to="net.P3V3" />
  <trace from="J_DBG.1" to="net.P3V3" />
  <trace from="J_PWR.2" to="net.GND" />
  <trace from="D1.anode" to="net.GND" />
  <trace from="C_BULK1.pin2" to="net.GND" />
  <trace from="C_BULK2.pin2" to="net.GND" />
  <trace from="C_BULK3.pin2" to="net.GND" />
  <trace from="U1.GND" to="net.GND" />
  <trace from="C1.pin2" to="net.GND" />
  <trace from="C2.pin2" to="net.GND" />
  <trace from="C3.pin2" to="net.GND" />
  <trace from="C_EN.pin2" to="net.GND" />
  <trace from="SW1.2" to="net.GND" />
  <trace from="SW2.2" to="net.GND" />
  <trace from="U3.GND" to="net.GND" />
  <trace from="C_RTC.pin2" to="net.GND" />
  <trace from="BT1.N" to="net.GND" />
  <trace from="U4.GND" to="net.GND" />
  <trace from="C_INA.pin2" to="net.GND" />
  <trace from="U4.A0" to="net.GND" />
  <trace from="U4.A1" to="net.GND" />
  <trace from="U5.GND" to="net.GND" />
  <trace from="C_U5.pin2" to="net.GND" />
  <trace from="R_LK_RB.pin2" to="net.GND" />
  <trace from="C_LK.pin2" to="net.GND" />
  <trace from="U5.IN2P" to="net.GND" />
  <trace from="U5.IN2N" to="net.GND" />
  <trace from="C_MOIST.pin2" to="net.GND" />
  <trace from="U7.GND" to="net.GND" />
  <trace from="C_IN5.pin2" to="net.GND" />
  <trace from="C_OUT5.pin2" to="net.GND" />
  <trace from="R_FB5B.pin2" to="net.GND" />
  <trace from="D_CAT5.anode" to="net.GND" />
  <trace from="U8.GND" to="net.GND" />
  <trace from="C_IN33.pin2" to="net.GND" />
  <trace from="C_OUT33.pin2" to="net.GND" />
  <trace from="Q1.source" to="net.GND" />
  <trace from="R1.pin2" to="net.GND" />
  <trace from="R_RP.pin2" to="net.GND" />
  <trace from="J_LED.2" to="net.GND" />
  <trace from="J_MOIST.2" to="net.GND" />
  <trace from="J_RES.2" to="net.GND" />
  <trace from="J_LEAK.2" to="net.GND" />
  <trace from="J_SENS.2" to="net.GND" />
  <trace from="J_STATUS.2" to="net.GND" />
  <trace from="J_DBG.4" to="net.GND" />
  <trace from="J_USB.GND" to="net.GND" />
  <trace from="J_USB.SHIELD" to="net.GND" />
  <trace from="U1.IO8" to="net.SDA" />
  <trace from="R_SDA.pin2" to="net.SDA" />
  <trace from="U3.SDA" to="net.SDA" />
  <trace from="U4.SDA" to="net.SDA" />
  <trace from="J_SENS.3" to="net.SDA" />
  <trace from="U1.IO9" to="net.SCL" />
  <trace from="R_SCL.pin2" to="net.SCL" />
  <trace from="U3.SCL" to="net.SCL" />
  <trace from="U4.SCL" to="net.SCL" />
  <trace from="J_SENS.4" to="net.SCL" />
  <trace from="U3.VBAT" to="net.VBAT" />
  <trace from="BT1.P" to="net.VBAT" />
  <trace from="U1.EN" to="net.EN" />
  <trace from="R_EN.pin2" to="net.EN" />
  <trace from="C_EN.pin1" to="net.EN" />
  <trace from="SW2.1" to="net.EN" />
  <trace from="U1.IO0" to="net.IO0_BOOT" />
  <trace from="SW1.1" to="net.IO0_BOOT" />
  <trace from="J_MOIST.3" to="net.MOIST_SIG" />
  <trace from="R_MOIST.pin1" to="net.MOIST_SIG" />
  <trace from="R_MOIST.pin2" to="net.MOIST_ADC" />
  <trace from="C_MOIST.pin1" to="net.MOIST_ADC" />
  <trace from="U1.IO4" to="net.MOIST_ADC" />
  <trace from="U1.IO5" to="net.RES_LOW_SW" />
  <trace from="J_RES.1" to="net.RES_LOW_SW" />
  <trace from="U1.IO6" to="net.RES_LEVEL_ADC" />
  <trace from="J_RES.3" to="net.RES_LEVEL_ADC" />
  <trace from="J_LEAK.1" to="net.LEAK_SENSE" />
  <trace from="R_LK_PU.pin2" to="net.LEAK_SENSE" />
  <trace from="U5.IN1P" to="net.LEAK_SENSE" />
  <trace from="C_LK.pin1" to="net.LEAK_SENSE" />
  <trace from="U5.IN1N" to="net.LEAK_REF" />
  <trace from="R_LK_RA.pin2" to="net.LEAK_REF" />
  <trace from="R_LK_RB.pin1" to="net.LEAK_REF" />
  <trace from="U5.OUT1" to="net.LEAK_DET" />
  <trace from="R_LK_OUT.pin2" to="net.LEAK_DET" />
  <trace from="U1.IO7" to="net.LEAK_DET" />
  <trace from="J_LED.4" to="net.LED_NTC" />
  <trace from="U1.IO2" to="net.LED_NTC" />
  <trace from="U1.IO10" to="net.PUMP_PWM" />
  <trace from="R2.pin1" to="net.PUMP_PWM" />
  <trace from="U1.IO14" to="net.LED_DIM" />
  <trace from="R_DIM.pin1" to="net.LED_DIM" />
  <trace from="R_DIM.pin2" to="net.LED_DIM_OUT" />
  <trace from="J_LED.3" to="net.LED_DIM_OUT" />
  <trace from="U1.IO21" to="net.STATUS_DATA" />
  <trace from="R_DATA.pin1" to="net.STATUS_DATA" />
  <trace from="R_DATA.pin2" to="net.STATUS_DATA_OUT" />
  <trace from="J_STATUS.3" to="net.STATUS_DATA_OUT" />
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
