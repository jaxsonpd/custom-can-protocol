import serial
from time import sleep
import threading

port = serial.Serial(port="/dev/ttyUSB0", baudrate=115200)

def read_thread():
    while True:
        data = port.read_all()
        if (data != b''):
            print(data)


read_thread = threading.Thread(target=read_thread)
read_thread.start()

sleep(2)
packet = bytes([126, 4, 0, 255, 255, 127])
packet2 = b'~\x01\x08\x00w\x01\x13\x00v\x00\x00i\x9b\x7f'

while(True):
    for x in packet2:
        port.write(bytes([x]))
        sleep(0.1)

    sleep(2)
    # port.write(packet)
    # sleep(2)