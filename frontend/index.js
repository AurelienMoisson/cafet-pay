var SerialPort = require('serialport')
var fs = require('fs')


var serialPort;
var i = 0
var done = false
while (!done) {
  done = true
  portPath = `/dev/ttyUSB${i}`
  if (!fs.existsSync(portPath)) {
    i += 1
    if (i<10) {
      done = false
    } else {
      console.log('unable to find serial Port')
      console.log('please ensure your arduino is correctly plugged in')
      throw new Error
    }
  }
}
serialPort = new SerialPort(portPath, {baudRate:9600})

console.log('connected to serial Port')


serialPort.on('data', (data) => {console.log('Data:', 0+data)})
