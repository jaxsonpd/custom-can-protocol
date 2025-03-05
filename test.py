import serial
from time import sleep
port = serial.Serial(port="/dev/ttyUSB0", baudrate=115200)

sleep(2)
packet = bytes([126, 4, 0, 255, 255, 127])
for byte in packet:
    port.write(byte)
    print(byte)
    sleep(0.2)

while(True):
    for byte in packet:
        port.write(byte)
        print(byte)
        sleep(0.2)
    readOut = port.read(1)
    if readOut != b"\xFF":
        print(readOut)