#! /usr/bin/python3
from serial import Serial
from sys import stdout
from accounts.link import *

for i in range(10):
    tty = "/dev/ttyUSB"+str(i)
    try:
        ser = Serial(tty, 9600)
    except:
        pass
    else:
        break
else:
    print("card reader not found")
    quit()


def bytes_to_long(s):
    out = 0
    for b in s:
        out = out*256 + b
    return out

while True:
    cardid = bytes_to_long(ser.readline())
    stdout.write(hex(cardid)+'\n')
    stdout.flush()
