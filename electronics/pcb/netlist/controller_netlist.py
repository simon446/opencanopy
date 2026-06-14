#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
controller_netlist.py — the OpenCanopy controller board's electrical design, as data.

This is the **single source of truth for the schematic** (WI-EE-03) at component + connectivity
level: every populated part, every net, pin by pin. It exists because the project has no KiCad GUI in
this environment — so the schematic is captured here as a reviewable, diff-able, CI-checkable netlist
instead of a binary `.kicad_sch`. From it we emit:

  * a flat netlist CSV (`--emit-csv`)            -> human/diff-friendly
  * a KiCad-importable netlist  (`--emit-kicad`)  -> `kicad-cli`/"Import Netlist" drives PCB layout

and we run an **ERC-style design-rule check** (`--check` / `--selftest`) that stands in for
`kicad-cli sch erc` until the KiCad source is entered (then ERC supersedes it). The check also
cross-validates the netlist against the committed pin map and the BOM, so the firmware pin contract
(`electronics/analysis/pin-map.csv`) and the buildable parts list (`electronics/bom/bom.csv`) can
never silently drift from the schematic.

Pin identifiers are **functional names** (e.g. `IO8`, `VIN`, `GND`, `IN+`), not datasheet pad numbers
— unambiguous, reviewable, and library-independent (KiCad maps symbol pin names on import). Two-pin
passives use `1`/`2`. Net names follow KiCad power convention (`GND`, `+3V3`, `+5V`, `+12V`, `+24V`).

No fan in V1 (ECO-001): the fan-drive parts are present as **DNP** so the board can drive a fan later
without a respin; their nets are flagged `reserved` and excluded from the floating-net rule.

Run:  python3 electronics/pcb/netlist/controller_netlist.py --selftest
"""
from __future__ import annotations

import argparse
import csv
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
PIN_MAP = REPO / "electronics" / "analysis" / "pin-map.csv"
BOM = REPO / "electronics" / "bom" / "bom.csv"

# ----------------------------------------------------------------------------- components
# ref -> (value, footprint-class, part_no, board, populated)
#   board:     "ctrl" controller PCB / "status" front-panel PCB
#   populated: True = fitted in V1 / False = DNP (footprint only)
P = True
DNP = False


class Comp:
    __slots__ = ("value", "footprint", "part", "board", "populated", "desc")

    def __init__(self, value, footprint, part, board="ctrl", populated=P, desc=""):
        self.value = value
        self.footprint = footprint
        self.part = part
        self.board = board
        self.populated = populated
        self.desc = desc


COMPONENTS: dict[str, Comp] = {
    # --- MCU + support ---
    "U1": Comp("ESP32-S3-WROOM-1", "RF_Module:ESP32-S3-WROOM-1", "ESP32-S3-WROOM-1-N8R8", desc="MCU module"),
    "C1": Comp("22uF", "C_0805", "GRM21-22uF-10V", desc="U1 3V3 bulk"),
    "C2": Comp("100nF", "C_0402", "C-100nF", desc="U1 decoupling"),
    "C3": Comp("100nF", "C_0402", "C-100nF", desc="U1 decoupling"),
    "R_EN": Comp("10k", "R_0402", "R-10k", desc="U1 EN pull-up"),
    "C_EN": Comp("100nF", "C_0402", "C-100nF", desc="U1 EN RC delay"),
    "SW1": Comp("BOOT", "SW_SPST_SMD", "SW-TACT", desc="boot/download button (IO0)"),
    "SW2": Comp("RST", "SW_SPST_SMD", "SW-TACT", desc="chip reset (EN)"),
    # --- I2C bus pull-ups ---
    "R_SDA": Comp("4.7k", "R_0402", "R-4k7", desc="I2C SDA pull-up to 3V3"),
    "R_SCL": Comp("4.7k", "R_0402", "R-4k7", desc="I2C SCL pull-up to 3V3"),
    # --- RTC (battery-backed) ---
    "U3": Comp("DS3231SN", "SOIC-16W", "DS3231SN", desc="RTC, I2C 0x68"),
    "C_RTC": Comp("100nF", "C_0402", "C-100nF", desc="U3 decoupling"),
    "BT1": Comp("CR2032", "BatteryHolder_CR2032", "CR2032+holder", desc="RTC backup cell"),
    # --- Pump current monitor ---
    "U4": Comp("INA219", "SOIC-8", "INA219AIDR", desc="pump current monitor, I2C 0x40"),
    "C_INA": Comp("100nF", "C_0402", "C-100nF", desc="U4 decoupling"),
    "R_SHUNT": Comp("0.1R", "R_2512", "R-0R1-1W", desc="pump current shunt (0.1 ohm)"),
    # --- Leak comparator ---
    "U5": Comp("LM393", "SOIC-8", "LM393DR", desc="leak comparator (dual; one used)"),
    "C_U5": Comp("100nF", "C_0402", "C-100nF", desc="U5 decoupling"),
    "R_LK_PU": Comp("100k", "R_0402", "R-100k", desc="leak-trace pull-up to 3V3"),
    "R_LK_RA": Comp("10k", "R_0402", "R-10k", desc="leak threshold divider top"),
    "R_LK_RB": Comp("10k", "R_0402", "R-10k", desc="leak threshold divider bottom"),
    "R_LK_OUT": Comp("10k", "R_0402", "R-10k", desc="LM393 open-collector pull-up"),
    "C_LK": Comp("100nF", "C_0402", "C-100nF", desc="leak debounce"),
    # --- Moisture front end ---
    "R_MOIST": Comp("1k", "R_0402", "R-1k", desc="moisture ADC series R"),
    "C_MOIST": Comp("10nF", "C_0402", "C-10nF", desc="moisture ADC RC filter"),
    # --- LED-heatsink NTC divider (optional, DR-09 secondary) ---
    "R_NTC": Comp("10k", "R_0402", "R-10k", populated=DNP, desc="NTC divider top (optional)"),
    # --- Power input + protection ---
    "F1": Comp("6.3A", "Fuseholder_5x20", "fuse-6p3A", desc="input fuse (8A for 150W build)"),
    "Q2": Comp("DMP3056L", "SOT-23", "DMP3056L", desc="reverse-polarity P-FET (high-side)"),
    "R_RP": Comp("100k", "R_0402", "R-100k", desc="Q2 gate resistor to GND"),
    "DZ_RP": Comp("15V", "SOD-323", "BZT52-15V", desc="Q2 gate clamp Zener"),
    "D1": Comp("SMBJ28A", "SMB", "SMBJ28A", desc="input TVS (28V standoff)"),
    "C_BULK1": Comp("100uF", "CP_Elec_8x10", "100uF-50V", desc="24V bulk electrolytic"),
    "C_BULK2": Comp("10uF", "C_1206", "10uF-50V", desc="24V bulk ceramic"),
    "C_BULK3": Comp("100nF", "C_0402", "C-100nF", desc="24V HF bypass"),
    # --- 24->12V buck (optional/DNP: 12V pump alternate only; no fan in V1) ---
    "U6": Comp("TPS5430", "SOIC-8", "TPS5430DDA", populated=DNP, desc="24->12V buck (optional)"),
    "C_IN12": Comp("10uF", "C_1206", "10uF-50V", populated=DNP, desc="U6 input cap"),
    "C_BT12": Comp("10nF", "C_0402", "C-10nF", populated=DNP, desc="U6 boot cap"),
    "L12": Comp("15uH", "L_12x12", "15uH-3A", populated=DNP, desc="U6 inductor"),
    "D_CAT12": Comp("B340A", "SMC", "B340A", populated=DNP, desc="U6 catch diode"),
    "C_OUT12": Comp("22uF", "C_1210", "22uF-25V", populated=DNP, desc="U6 output cap"),
    "R_FB12A": Comp("68k", "R_0402", "R-68k", populated=DNP, desc="U6 feedback top"),
    "R_FB12B": Comp("7.5k", "R_0402", "R-7k5", populated=DNP, desc="U6 feedback bottom"),
    # --- 24->5V buck ---
    "U7": Comp("TPS5430", "SOIC-8", "TPS5430DDA", desc="24->5V buck"),
    "C_IN5": Comp("10uF", "C_1206", "10uF-50V", desc="U7 input cap"),
    "C_BT5": Comp("10nF", "C_0402", "C-10nF", desc="U7 boot cap"),
    "L5": Comp("10uH", "L_12x12", "10uH-3A", desc="U7 inductor"),
    "D_CAT5": Comp("B340A", "SMC", "B340A", desc="U7 catch diode"),
    "C_OUT5": Comp("22uF", "C_1210", "22uF-25V", desc="U7 output cap"),
    "R_FB5A": Comp("22k", "R_0402", "R-22k", desc="U7 feedback top"),
    "R_FB5B": Comp("7.15k", "R_0402", "R-7k15", desc="U7 feedback bottom"),
    # --- 5->3.3V LDO ---
    "U8": Comp("AP2112K-3.3", "SOT-23-5", "AP2112K-3.3", desc="3V3 LDO for MCU/logic"),
    "C_IN33": Comp("1uF", "C_0402", "C-1uF", desc="U8 input cap"),
    "C_OUT33": Comp("1uF", "C_0402", "C-1uF", desc="U8 output cap"),
    # --- Pump drive (fail-OFF) ---
    "Q1": Comp("DMN3404L", "SOT-23", "DMN3404L", desc="pump N-FET, low-side"),
    "R1": Comp("10k", "R_0402", "R-10k", desc="pump gate->GND pull-down (FAIL-OFF)"),
    "R2": Comp("100R", "R_0402", "R-100R", desc="pump gate series R"),
    "D2": Comp("SS34", "SMC", "SS34", desc="pump flyback Schottky"),
    # --- LED dim (PWM-direct populated; 0-10V op-amp path DNP) ---
    "R_DIM": Comp("100R", "R_0402", "R-100R", desc="LED dim series R"),
    "U9": Comp("MCP6001", "SOT-23-5", "MCP6001", populated=DNP, desc="0-10V dim buffer (optional)"),
    "R_DIMF": Comp("10k", "R_0402", "R-10k", populated=DNP, desc="dim RC filter R (optional)"),
    "C_DIMF": Comp("1uF", "C_0402", "C-1uF", populated=DNP, desc="dim RC filter C (optional)"),
    # --- Fan drive (DNP: no fan in V1, ECO-001) — 4-pin PWM fan provision (PWM in + tach out) ---
    "R_FANG": Comp("100R", "R_0402", "R-100R", populated=DNP, desc="fan PWM series R (DNP)"),
    "R_TACH": Comp("10k", "R_0402", "R-10k", populated=DNP, desc="fan tach pull-up (DNP)"),
    # --- Status data series R (on controller side of J_STATUS) ---
    "R_DATA": Comp("330R", "R_0402", "R-330R", desc="WS2812 data series R"),
    # --- Field connectors ---
    "J_PWR": Comp("XT30", "XT30PW", "XT30PW", desc="24V power input"),
    "J_LED": Comp("JST_VH_4", "JST_VH_B4PS", "B4PS-VH", desc="grow light (24V/GND/DIM/NTC)"),
    "J_PUMP": Comp("JST_VH_2", "JST_VH_B2PS", "B2PS-VH", desc="pump (24V/return)"),
    "J_FAN": Comp("FAN_4", "Fan_4pin", "47053-1000", populated=DNP, desc="fan header (DNP, no fan V1)"),
    "J_MOIST": Comp("JST_PH_3", "JST_PH_B3B", "B3B-PH", desc="moisture probe"),
    "J_RES": Comp("JST_PH_3", "JST_PH_B3B", "B3B-PH", desc="reservoir level"),
    "J_LEAK": Comp("JST_PH_2", "JST_PH_B2B", "B2B-PH", desc="leak sensor"),
    "J_SENS": Comp("JST_PH_4", "JST_PH_B4B", "B4B-PH", desc="temp/RH (SHT40) + I2C"),
    "J_STATUS": Comp("JST_XH_3", "JST_XH_B3B", "B3B-XH", desc="status LED board"),
    "J_DBG": Comp("Header_4", "PinHeader_1x04", "header-4", desc="debug UART"),
    "J_USB": Comp("USB-C", "USB_C_Receptacle", "USB4105", desc="USB-CDC flash/log"),
    "J_EXP": Comp("Header_6", "PinHeader_1x06", "header-6", populated=DNP, desc="expansion (HX711/PAR/pH-EC)"),
    "J_CAM": Comp("Header_2", "PinHeader_1x02", "header-2", populated=DNP, desc="camera (unpopulated)"),
    # --- Status LED board (separate PCB2) ---
    "LED1": Comp("WS2812B-2020", "LED_WS2812B-2020", "WS2812B-2020", board="status", desc="status pos 1"),
    "LED2": Comp("WS2812B-2020", "LED_WS2812B-2020", "WS2812B-2020", board="status", desc="status pos 2"),
    "LED3": Comp("WS2812B-2020", "LED_WS2812B-2020", "WS2812B-2020", board="status", desc="status pos 3"),
    "LED4": Comp("WS2812B-2020", "LED_WS2812B-2020", "WS2812B-2020", board="status", desc="status pos 4"),
    "LED5": Comp("WS2812B-2020", "LED_WS2812B-2020", "WS2812B-2020", board="status", desc="status pos 5"),
    "C_L1": Comp("100nF", "C_0402", "C-100nF", board="status", desc="LED1 decoupling"),
    "C_L2": Comp("100nF", "C_0402", "C-100nF", board="status", desc="LED2 decoupling"),
    "C_L3": Comp("100nF", "C_0402", "C-100nF", board="status", desc="LED3 decoupling"),
    "C_L4": Comp("100nF", "C_0402", "C-100nF", board="status", desc="LED4 decoupling"),
    "C_L5": Comp("100nF", "C_0402", "C-100nF", board="status", desc="LED5 decoupling"),
    "C_LBULK": Comp("10uF", "C_0805", "10uF-10V", board="status", desc="status board bulk"),
    "D_LVD": Comp("1N5817", "SOD-123", "1N5817", board="status", desc="series diode: drop 5V->~4.5V for WS2812 VIH"),
    "J_STA": Comp("JST_XH_3", "JST_XH_B3B", "B3B-XH", board="status", desc="cable from controller"),
}

# ----------------------------------------------------------------------------- nets
# net -> list of "REF.PIN".  Functional pin names.  KiCad power-net naming.
NETS: dict[str, list[str]] = {
    # ---------- controller board ----------
    # Power input chain: J_PWR -> F1 -> Q2 (reverse-pol) -> +24V protected rail
    "+24V_IN": ["J_PWR.1", "F1.1"],
    "+24V_FUSED": ["F1.2", "Q2.S", "DZ_RP.K"],
    "RP_GATE": ["Q2.G", "DZ_RP.A", "R_RP.1"],
    "+24V": [
        "Q2.D", "D1.K", "C_BULK1.+", "C_BULK2.1", "C_BULK3.1",
        "U6.VIN", "U6.ENA", "C_IN12.1", "U7.VIN", "U7.ENA", "C_IN5.1",
        "J_LED.1", "R_SHUNT.1", "U4.IN+",
    ],
    # Pump high-side current shunt then connector then low-side FET to GND
    "PUMP_+": ["R_SHUNT.2", "U4.IN-", "J_PUMP.1", "D2.K"],
    "PUMP_RET": ["J_PUMP.2", "Q1.D", "D2.A"],
    "PUMP_GATE": ["Q1.G", "R2.2", "R1.1"],
    # 12V buck (DNP / optional 12V pump)
    "PH12": ["U6.PH", "L12.1", "D_CAT12.K", "C_BT12.2"],
    "+12V": ["L12.2", "C_OUT12.1", "R_FB12A.1", "J_FAN.2"],
    "FB12": ["U6.VSENSE", "R_FB12A.2", "R_FB12B.1"],
    "BOOT12": ["U6.BOOT", "C_BT12.1"],
    # 5V buck
    "PH5": ["U7.PH", "L5.1", "D_CAT5.K", "C_BT5.2"],
    "+5V": ["L5.2", "C_OUT5.1", "R_FB5A.1", "U8.VIN", "U8.EN", "C_IN33.1", "J_STATUS.1"],
    "FB5": ["U7.VSENSE", "R_FB5A.2", "R_FB5B.1"],
    "BOOT5": ["U7.BOOT", "C_BT5.1"],
    # 3V3 LDO
    "+3V3": [
        "U8.VOUT", "C_OUT33.1",
        "U1.3V3", "C1.1", "C2.1", "C3.1", "R_EN.1",
        "R_SDA.1", "R_SCL.1",
        "U3.VCC", "C_RTC.1",
        "U4.VS", "C_INA.1",
        "U5.VCC", "C_U5.1", "R_LK_PU.1", "R_LK_RA.1", "R_LK_OUT.1",
        "R_NTC.1", "R_TACH.2",
        "J_MOIST.1", "J_SENS.1", "J_DBG.1",
    ],
    # Ground (single logical net; KiCad symbol has several GND pads -> same net)
    "GND": [
        "J_PWR.2", "D1.A", "C_BULK1.-", "C_BULK2.2", "C_BULK3.2",
        "U1.GND", "C1.2", "C2.2", "C3.2", "C_EN.2",
        "SW1.2", "SW2.2",
        "U3.GND", "C_RTC.2", "BT1.-",
        "U4.GND", "C_INA.2", "U4.A0", "U4.A1",
        "U5.GND", "C_U5.2", "R_LK_RB.2", "C_LK.2", "U5.IN2+", "U5.IN2-",
        "C_MOIST.2",
        "U6.GND", "C_IN12.2", "C_OUT12.2", "R_FB12B.2", "D_CAT12.A",
        "U7.GND", "C_IN5.2", "C_OUT5.2", "R_FB5B.2", "D_CAT5.A",
        "U8.GND", "C_IN33.2", "C_OUT33.2",
        "Q1.S",
        "R1.2", "R_RP.2",
        "U9.V-", "C_DIMF.2",
        "J_LED.2", "J_FAN.1", "J_MOIST.2", "J_RES.2", "J_LEAK.2",
        "J_SENS.2", "J_STATUS.2", "J_DBG.4", "J_USB.GND", "J_USB.SHIELD",
    ],
    # I2C bus
    "SDA": ["U1.IO8", "R_SDA.2", "U3.SDA", "U4.SDA", "J_SENS.3"],
    "SCL": ["U1.IO9", "R_SCL.2", "U3.SCL", "U4.SCL", "J_SENS.4"],
    # RTC backup
    "VBAT": ["U3.VBAT", "BT1.+"],
    # MCU EN / boot
    "EN": ["U1.EN", "R_EN.2", "C_EN.1", "SW2.1"],
    "IO0_BOOT": ["U1.IO0", "SW1.1"],
    # Moisture ADC front end
    "MOIST_SIG": ["J_MOIST.3", "R_MOIST.1"],
    "MOIST_ADC": ["R_MOIST.2", "C_MOIST.1", "U1.IO4"],
    # Reservoir
    "RES_LOW_SW": ["U1.IO5", "J_RES.1"],
    "RES_LEVEL_ADC": ["U1.IO6", "J_RES.3"],
    # Leak comparator: trace -> +in (pull-up), divider -> -in, OC out -> IO7
    "LEAK_SENSE": ["J_LEAK.1", "R_LK_PU.2", "U5.IN1+", "C_LK.1"],
    "LEAK_REF": ["U5.IN1-", "R_LK_RA.2", "R_LK_RB.1"],
    "LEAK_DET": ["U5.OUT1", "R_LK_OUT.2", "U1.IO7"],
    # LED heatsink NTC (optional) divider to IO2 (R_NTC.1 -> +3V3 above)
    "LED_NTC": ["J_LED.4", "U1.IO2", "R_NTC.2"],
    # Pump drive
    "PUMP_PWM": ["U1.IO10", "R2.1"],
    # LED dim (PWM-direct populated; unity-buffer op-amp path is DNP, output ties to IN-)
    "LED_DIM": ["U1.IO14", "R_DIM.1"],
    "LED_DIM_OUT": ["R_DIM.2", "J_LED.3", "U9.OUT", "U9.IN-"],
    "DIM_FILT": ["R_DIMF.2", "C_DIMF.1", "U9.IN+"],
    "DIM_PWM_TAP": ["R_DIMF.1"],  # PWM tap if op-amp path fitted (DNP)
    "U9_VDD": ["U9.V+"],  # +5V if fitted (DNP)
    # Status LED data
    "STATUS_DATA": ["U1.IO21", "R_DATA.1"],
    "STATUS_DATA_OUT": ["R_DATA.2", "J_STATUS.3"],
    # USB / UART
    "USB_DM": ["U1.IO19", "J_USB.D-"],
    "USB_DP": ["U1.IO20", "J_USB.D+"],
    "VBUS": ["J_USB.VBUS"],
    "UART_TX": ["U1.IO43", "J_DBG.2"],
    "UART_RX": ["U1.IO44", "J_DBG.3"],
    # Fan drive (DNP, reserved) — 4-pin PWM fan: PWM out to fan, tach in (pull-up to 3V3)
    "FAN_PWM": ["U1.IO12", "R_FANG.1"],
    "FAN_PWM_OUT": ["R_FANG.2", "J_FAN.3"],
    "FAN_TACH": ["U1.IO13", "R_TACH.1", "J_FAN.4"],
    # Expansion (DNP headers)
    "EXP_HX711_DT": ["U1.IO15", "J_EXP.1"],
    "EXP_HX711_SCK": ["U1.IO16", "J_EXP.2"],
    "EXP_GP1": ["U1.IO17", "J_EXP.3"],
    "EXP_GP2": ["U1.IO18", "J_EXP.4"],
    "EXP_CAM1": ["U1.IO47", "J_CAM.1"],
    "EXP_CAM2": ["U1.IO48", "J_CAM.2"],

    # ---------- status LED board (PCB2) ----------
    "S_+5V": ["J_STA.1", "D_LVD.A", "C_LBULK.1"],
    "S_VLED": ["D_LVD.K", "LED1.VDD", "LED2.VDD", "LED3.VDD", "LED4.VDD", "LED5.VDD",
               "C_L1.1", "C_L2.1", "C_L3.1", "C_L4.1", "C_L5.1"],
    "S_GND": ["J_STA.2", "C_LBULK.2", "LED1.GND", "LED2.GND", "LED3.GND", "LED4.GND", "LED5.GND",
              "C_L1.2", "C_L2.2", "C_L3.2", "C_L4.2", "C_L5.2"],
    "S_DIN": ["J_STA.3", "LED1.DIN"],
    "S_D12": ["LED1.DOUT", "LED2.DIN"],
    "S_D23": ["LED2.DOUT", "LED3.DIN"],
    "S_D34": ["LED3.DOUT", "LED4.DIN"],
    "S_D45": ["LED4.DOUT", "LED5.DIN"],
    "S_D5OUT": ["LED5.DOUT"],  # chain end (no-connect)
}

# Nets that are *allowed* to have a single connected pin (reserved/no-connect/DNP-only), with reason.
SINGLE_PIN_OK = {
    "VBUS": "USB VBUS not used to power the board (bus-powered flashing only)",
    "S_D5OUT": "WS2812 chain end (DOUT of last LED is a no-connect)",
    "DIM_PWM_TAP": "0-10V op-amp dim path is DNP (PWM-direct dim populated)",
    "U9_VDD": "op-amp supply — DNP path",
}

# DNP-only/reserved nets excluded from the >=2-active-pin rule (their parts are DNP).
RESERVED_NETS = {
    "PH12", "+12V", "FB12", "BOOT12",            # 12V buck DNP
    "DIM_FILT", "U9_VDD", "DIM_PWM_TAP",         # op-amp dim DNP
    "FAN_PWM", "FAN_PWM_OUT", "FAN_TACH",        # fan DNP (4-pin PWM provision)
    "EXP_HX711_DT", "EXP_HX711_SCK", "EXP_GP1", "EXP_GP2", "EXP_CAM1", "EXP_CAM2",  # expansion DNP
    "LED_NTC",                                   # optional NTC
}


def _pin_ref(pin: str) -> str:
    return pin.split(".", 1)[0]


def _norm_pins() -> dict[str, list[str]]:
    """Drop the '.../x' duplicate-pad disambiguators used so one physical pad can sit on two
    logical lines in this functional model (e.g. a connector GND pad). Returns net->pins."""
    out = {}
    for net, pins in NETS.items():
        cleaned = []
        for p in pins:
            ref, _, pinname = p.partition(".")
            pinname = pinname.rstrip("x")  # 'J_PUMP.1x' -> connector GND pad alias
            cleaned.append(f"{ref}.{pinname}")
        out[net] = cleaned
    return out


# ----------------------------------------------------------------------------- emit
def emit_csv() -> str:
    rows = ["net,ref,pin,populated,part"]
    for net, pins in NETS.items():
        for p in pins:
            ref, _, pin = p.partition(".")
            c = COMPONENTS.get(ref)
            pop = "Y" if (c and c.populated) else ("DNP" if c else "?")
            part = c.part if c else "?"
            rows.append(f"{net},{ref},{pin},{pop},{part}")
    return "\n".join(rows) + "\n"


def emit_kicad() -> str:
    """A minimal KiCad-style flat netlist (importable to seed PCB layout)."""
    out = ["(export (version D)", "  (components"]
    for ref, c in COMPONENTS.items():
        dnp = " (property (name \"dnp\") (value \"1\"))" if not c.populated else ""
        out.append(f"    (comp (ref \"{ref}\") (value \"{c.value}\") "
                   f"(footprint \"{c.footprint}\") (datasheet \"{c.part}\"){dnp})")
    out.append("  )")
    out.append("  (nets")
    for i, (net, pins) in enumerate(_norm_pins().items(), start=1):
        out.append(f"    (net (code \"{i}\") (name \"{net}\")")
        for p in pins:
            ref, _, pin = p.partition(".")
            out.append(f"      (node (ref \"{ref}\") (pin \"{pin}\"))")
        out.append("    )")
    out.append("  )")
    out.append(")")
    return "\n".join(out) + "\n"


# ----------------------------------------------------------------------------- ERC check
def _read_pinmap_nets() -> dict[str, str]:
    """net-name -> esp32 gpio, from pin-map.csv (the firmware pin contract)."""
    m = {}
    if not PIN_MAP.exists():
        return m
    with PIN_MAP.open() as f:
        for row in csv.DictReader(f):
            m[row["net"].strip()] = row["esp32s3_gpio"].strip()
    return m


def _read_bom_refs() -> set[str]:
    """All designators in the BOM. A BOM row's `ref` cell may list several space-separated
    designators (grouped passives), so split it."""
    refs = set()
    if not BOM.exists():
        return refs
    with BOM.open() as f:
        for row in csv.DictReader(f):
            for d in row["ref"].split():
                refs.add(d.strip())
    return refs


# pin-map net -> (this netlist's net name, expected MCU pin)  [firmware contract cross-check]
PINMAP_TO_NET = {
    "I2C_SDA": ("SDA", "IO8"), "I2C_SCL": ("SCL", "IO9"),
    "MOISTURE_ADC": ("MOIST_ADC", "IO4"),
    "RES_LOW_SW": ("RES_LOW_SW", "IO5"), "RES_LEVEL_ADC": ("RES_LEVEL_ADC", "IO6"),
    "LEAK_DET": ("LEAK_DET", "IO7"), "LED_HS_NTC": ("LED_NTC", "IO2"),
    "PUMP_PWM": ("PUMP_PWM", "IO10"),
    "FAN_PWM": ("FAN_PWM", "IO12"), "FAN_TACH": ("FAN_TACH", "IO13"),
    "LED_DIM_PWM": ("LED_DIM", "IO14"), "STATUS_LED_DATA": ("STATUS_DATA", "IO21"),
    "UART0_TX": ("UART_TX", "IO43"), "UART0_RX": ("UART_RX", "IO44"),
    "USB_DM": ("USB_DM", "IO19"), "USB_DP": ("USB_DP", "IO20"),
    "EXP_HX711_DT": ("EXP_HX711_DT", "IO15"), "EXP_HX711_SCK": ("EXP_HX711_SCK", "IO16"),
    "EXP_GP1": ("EXP_GP1", "IO17"), "EXP_GP2": ("EXP_GP2", "IO18"),
    "EXP_CAM1": ("EXP_CAM1", "IO47"), "EXP_CAM2": ("EXP_CAM2", "IO48"),
}


def erc_check() -> list[str]:
    errors: list[str] = []
    nets = _norm_pins()

    # 1. every pin's ref exists
    for net, pins in nets.items():
        for p in pins:
            ref = _pin_ref(p)
            if ref not in COMPONENTS:
                errors.append(f"net {net}: pin {p} references unknown component {ref}")

    # 2. no pin assigned to two different nets
    seen: dict[str, str] = {}
    for net, pins in nets.items():
        for p in pins:
            if p in seen and seen[p] != net:
                errors.append(f"pin {p} is in two nets: {seen[p]} and {net}")
            seen[p] = net

    # 3. floating nets: >=2 connected pins, unless explicitly single-pin-ok or a reserved/DNP net
    for net, pins in nets.items():
        active = [p for p in pins if COMPONENTS.get(_pin_ref(p)) and COMPONENTS[_pin_ref(p)].populated]
        if net in RESERVED_NETS:
            continue
        if len(active) < 2 and net not in SINGLE_PIN_OK:
            errors.append(f"net {net}: only {len(active)} populated pin(s) — floating? "
                          f"(add to SINGLE_PIN_OK/RESERVED_NETS if intentional)")

    # 4. every populated component appears on >=1 net
    on_net = {_pin_ref(p) for pins in nets.values() for p in pins}
    for ref, c in COMPONENTS.items():
        if c.populated and ref not in on_net:
            errors.append(f"component {ref} ({c.value}) is populated but on no net (orphan)")

    # 5. core power nets exist and are populated
    for rail in ("+24V", "+5V", "+3V3", "GND"):
        if rail not in nets:
            errors.append(f"missing core power net {rail}")

    # 6. firmware pin contract: pin-map.csv GPIO == this netlist's MCU pin
    pm = _read_pinmap_nets()
    for pmnet, (mynet, mypin) in PINMAP_TO_NET.items():
        gpio = pm.get(pmnet, "")
        want = "GPIO" + mypin[2:] if mypin.startswith("IO") else mypin
        if gpio and gpio != want:
            errors.append(f"pin-map {pmnet}={gpio} but netlist {mynet} uses U1.{mypin} (={want})")
        if mynet not in nets or f"U1.{mypin}" not in nets.get(mynet, []):
            errors.append(f"netlist net {mynet} missing U1.{mypin} (pin-map {pmnet})")

    # 7. BOM coverage: EVERY populated controller-board netlist component must appear in bom.csv
    #    (connectors carry a CN_* alias; everything else matches by ref). This is what stops the
    #    schematic and the buildable parts list from silently drifting.
    bom_refs = _read_bom_refs()
    for r, c in COMPONENTS.items():
        if c.board != "ctrl" or not c.populated:
            continue
        bom_ref = BOM_REF_MAP.get(r, r)
        if bom_ref not in bom_refs:
            errors.append(f"netlist {r} ({c.value}) -> BOM ref '{bom_ref}' missing from bom.csv")

    return errors


# Netlist connector refs (J_*) carry a CN_* alias in the BOM; everything else matches by ref.
BOM_REF_MAP = {
    "J_PWR": "CN_PWR", "J_LED": "CN_LED", "J_PUMP": "CN_PUMP", "J_FAN": "CN_FAN",
    "J_MOIST": "CN_MOIST", "J_RES": "CN_RES", "J_LEAK": "CN_LEAK", "J_SENS": "CN_SENS",
    "J_STATUS": "CN_STATUS", "J_DBG": "CN_DBG", "J_USB": "CN_USB",
}


# ----------------------------------------------------------------------------- selftest / CLI
def selftest() -> int:
    errs = erc_check()
    # Design sanity assertions independent of the live data, so the checker itself is proven.
    checks_ok = True

    # The fail-OFF guarantee: pump gate must see a pull-down to GND and a series R to the MCU.
    if "R1.1" not in NETS["PUMP_GATE"] or "Q1.G" not in NETS["PUMP_GATE"]:
        print("[selftest] FAIL: pump gate pull-down (R1) not on PUMP_GATE"); checks_ok = False
    if "R1.2" not in NETS["GND"]:
        print("[selftest] FAIL: pump gate pull-down R1 not tied to GND (fail-OFF broken)"); checks_ok = False

    # No fan in V1: every fan part must be DNP.
    for r in ("R_FANG", "R_TACH", "J_FAN"):
        if COMPONENTS[r].populated:
            print(f"[selftest] FAIL: fan part {r} is populated (must be DNP, ECO-001)"); checks_ok = False

    # I2C devices all on the bus.
    for dev in ("U3.SDA", "U4.SDA"):
        if dev not in NETS["SDA"]:
            print(f"[selftest] FAIL: {dev} not on SDA"); checks_ok = False

    if errs:
        checks_ok = False
        print(f"[selftest] ERC found {len(errs)} issue(s):")
        for e in errs:
            print(f"  - {e}")

    # Committed generated artifacts must match the source (no silent drift).
    for fname, emit in (("controller-netlist.csv", emit_csv), ("controller.net", emit_kicad)):
        f = HERE / fname
        if f.exists() and f.read_text(encoding="utf-8") != emit():
            print(f"[selftest] FAIL: {fname} is stale — run "
                  f"`controller_netlist.py --emit-{'csv' if 'csv' in fname else 'kicad'}`")
            checks_ok = False

    print("[selftest]", "ALL PASSED" if checks_ok else "FAILED")
    return 0 if checks_ok else 1


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description="OpenCanopy controller netlist (schematic-as-data).")
    ap.add_argument("--selftest", action="store_true", help="run ERC + design assertions (CI gate)")
    ap.add_argument("--check", action="store_true", help="run ERC against the live netlist")
    ap.add_argument("--emit-csv", action="store_true", help="write controller-netlist.csv")
    ap.add_argument("--emit-kicad", action="store_true", help="write controller.net (KiCad import)")
    ap.add_argument("--stats", action="store_true", help="print component/net counts")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()
    if args.check:
        errs = erc_check()
        for e in errs:
            print("ERC:", e)
        print("ERC:", "clean" if not errs else f"{len(errs)} issue(s)")
        return 1 if errs else 0
    if args.emit_csv:
        (HERE / "controller-netlist.csv").write_text(emit_csv(), encoding="utf-8")
        print("wrote controller-netlist.csv")
        return 0
    if args.emit_kicad:
        (HERE / "controller.net").write_text(emit_kicad(), encoding="utf-8")
        print("wrote controller.net")
        return 0
    if args.stats:
        pop = sum(1 for c in COMPONENTS.values() if c.populated)
        dnp = sum(1 for c in COMPONENTS.values() if not c.populated)
        print(f"components: {len(COMPONENTS)} ({pop} populated, {dnp} DNP)")
        print(f"nets: {len(NETS)}")
        return 0
    ap.print_help()
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
