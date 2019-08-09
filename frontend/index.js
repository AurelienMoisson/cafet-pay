var SerialPort = require('serialport')

portPath = '/dev/ttyUSB0'

var serialPort = new SerialPort(portPath, {baudRate:9600})

serialPort.on('data', (data) => {console.log('Data:', data)})
