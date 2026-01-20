import serial
import crcmod

## crc16 = crcmod.predefined.mkPredefinedCrcFun('modbus')
## 
## # open port
## ser = serial.Serial('/dev/ttyUSB0', 9600, bytesize=8, parity='N', stopbits=1, timeout=1)
## 
## frame = bytearray([0x01, 0xFA, 0x0E, 0x01, 0x00])
## crc = crc16(frame)
## frame += crc.to_bytes(2, byteorder='little')
## 
## ser.write(frame)
## 
## response = ser.read(256)
## print(response.hex())

# from pymodbus.client.sync import ModbusSerialClient as ModbusClient
# 
# mc = ModbusClient(method='rtu', port='/dev/ttyUSB0')
# 
# mc.connect()
# 
# from pymodbus import mei_message
# 
# rq = mei_message.ReadDeviceInformationRequest(unit=5,read_code=0x03)
# 
# rr = mc.execute(rq)
# 
# rr.information

# import serial.tools.list_ports as portlist
# for port in portlist.comports():
#     print(port)
#     # print(port.device)
    
import serial.tools.list_ports

for port in serial.tools.list_ports.comports():
    print(f"Device: {port.device}")
    print(f"  VID: {hex(port.vid) if port.vid else None}")
    print(f"  PID: {hex(port.pid) if port.pid else None}")
    print(f"  Serial: {port.serial_number}")