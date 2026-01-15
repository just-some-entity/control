# Minimal Example — LED Control on the WAGO 750-530
A complete hardware + software walkthrough

---

## Table of Contents
1. [Introduction](#1-introduction)  
2. [Requirements](#2-requirements)  
3. [Hardware Setup](#3-hardware-setup)  
4. [Software Setup](#4-software-setup)  
5. [Demo](#5-demo)  
6. [References](#6-references)  
7. [Acknowledgements](#7-acknowledgements)

---

## 1. Introduction

The WAGO 750-530 LED Toggle Example is a minimal demonstration showing how to control digital outputs on a **WAGO 750-530 EtherCAT terminal** using the QiTech machine framework.  
It represents the simplest possible hardware interaction in the system:  
**toggling LED outputs using the QiTech Control Dashboard.**

---

## 2. Requirements

### Hardware
- WAGO **750-354 Fieldbus Coupler EtherCAT**
- WAGO **750-602 Power Supply**
- WAGO **750-530 8-channel digital output**
- WAGO **750-600 End Module**
- **24 V DC power supply** (AC/DC adapter + DC hollow plug)  
- Jumper / bridge wires (0.5–1.5 mm² recommended)  
- Standard Ethernet cable  
- Flat screwdriver  
- A **Linux PC** (Ubuntu/Debian recommended)

### Software  
*(Installation steps in Section 4)*  
- Rust toolchain  
- Node.js + npm  
- Git  
- QiTech Control repository  
- EtherCAT HAL (included inside repo)

---

## 3. Hardware Setup

### 3.1 Wiring and Safety

#### Safe Wiring Procedure

1. Insert a screwdriver **straight** into the square release hole.  
2. Insert the stripped wire into the round opening.  
3. Remove the screwdriver — the spring clamp locks the wire.

![](../assets/wiring.png)

#### ⚠️ Safety Warning  
Always disconnect power before wiring.  
Working on live EtherCAT terminals can cause serious damage or electrical shock.

---

### 3.2 Schematic

![](../assets/schematic.png)

---

### 3.3 750-602 Wiring

This wiring configuration supplies the 750-602 with power 

It is not the only possible wiring but is the **simplest functional setup**.

### 3.3.1 Wiring Steps

Perform the following wiring on the 750-602:
1. Red wire **(+24 V)** → Terminal **6**
2. Black wire **(0 V)** → Terminal **7**

---

#### **Figure 1 — 750-602 Minimal wiring**
<img src="image" width="400">

### 3.4 750-354 Integration and Wiring

Slide the 750-354 onto the **left side of the 750-602** until it locks.
The EtherCAT K-Bus contacts connect automatically - **no wiring required**

### 3.4.1 Wiring Steps

Perform the following wiring from the 750-602 to the 750-354 using jumper cables: 

1. Terminal **2** → Terminal **Red**
2. Terminal **3** → Terminal **Blue**  

#### **Figure 2 — 750-602 and 750-354 connected and wired**
<img src="image" width="400">

---

### 3.5 750-530 Integration

Slide the 750-530 onto the **right side of the 750-602** until it locks.
The EtherCAT K-Bus and power contacts connect automatically - **no wiring required**

#### **Figure 3 — 750-530 Digital Output**
<img src="image" width="400">

#### **Figure 4 — Resulting setup**
<img src="image" width="400">

--- 

### 3.6 750-600 Integration

Slide the 750-600 onto the **right side of the 750-530** until it locks.
The EtherCAT K-Bus and power contacts connect automatically - **no wiring required**

#### **Figure 5 — 750-600 End Module**
<img src="image" width="400">

#### **Figure 6 — Attaching the 750-600 Module**
<img src="image" width="400">

---

### 3.8 Power and Ethernet

## 3.8.1 Power

Connect the 24 V adapter to the hollow plug used earlier.

#### **Figure 7 — Example AC/DC Adapter**
<img src="image" width="400">

---

### 3.8.2 Ethernet

Use a standard LAN cable to connect your PC → 750-354

#### **Figure 8 — Connecting a PC to the 750-354 via LAN cable**
<img src="image" width="400">

---

### 3.9 Final Assembled Setup

#### **Figure 9 — Final Assembled Setup**
<img src="image" width="400">

---

## 4. Software Setup

### 4.1 Installing on Ubuntu/Debian

Paste this into your terminal:

```bash
# Press Enter when prompted
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt update
sudo apt install -y npm nodejs git

git clone git@github.com:qitechgmbh/control.git
cd control/electron
npm install
```
### 4.2 Running the Backend
```bash
./cargo_run_linux.sh
```
This script:

- Builds the backend

- Grants required system capabilities (raw sockets)

- Starts EtherCAT communication

Ensure the 750-354 is connected before running the script.

### 4.3 Running the Frontend
```bash
cd electron
npm run start
```
This launches the QiTech Control dashboard.

## 5. Demo

### 5.1 Assigning Devices in the Dashboard

Once the backend + frontend are running, you should see:

- $DEVICE

- $DEVICE
![](../assets/discovery.png)

Steps:

- Click Assign on the $DEVICE

- Select TestMachine V1
 
![](../assets/setmachine.png)


- Enter a serial number (use the same for $DEVICE + $DEVICE)

![](../assets/serserial.png)

- Click Write

- Repeat for the $DEVICE

### 5.2 Testing LED Control
Navigate to:

Machines → TestMachine
![](../assets/machinedetected.png)
You will see this interface:

![](../assets/machinecontrol.png)

**You can now toggle the eight digital outputs of the 750-530.**

## 6. References

This guide incorporates information from official WAGO documentation.
All diagrams, product names, and figures belong to WAGO GmbH & Co. KG and are used here solely for educational purposes.

Referenced Manuals

[WAGO 750-354 Product Manual](https://www.wago.com/medias/m07500354-00000000-0en.pdf)

[WAGO 750-354 Product Manual](https://www.wago.com/medias/m07500354-00000000-0en.pdf)

[WAGO 750-354 Product Manual](https://www.wago.com/medias/m07500354-00000000-0en.pdf)

[WAGO 750-354 Product Manual](https://www.wago.com/medias/m07500354-00000000-0en.pdf)

## 7. Acknowledgements

This tutorial is inspired by the clarity and educational quality of WAGO manuals.
All wiring illustrations and hardware descriptions in this guide are provided for demonstration purposes only and do not replace official WAGO installation guidelines.

Special thanks to the QiTech engineering team for providing the backend architecture, EtherCAT HAL abstraction, and the TestMachine framework that makes this example possible.