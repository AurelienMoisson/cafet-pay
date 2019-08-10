var fs = require('fs')
var SerialPort = require('serialport')

function connect() {
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
        return null
      }
    }
  }
  return new SerialPort(portPath, {baudRate:9600})
}
module.exports = {connect}
