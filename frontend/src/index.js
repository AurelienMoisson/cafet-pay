const serial = require('./serial')

const device = serial.connect()
if (device) {
  console.log('connected to serial Port')

  device.on('data', (data) => {console.log('Data:', data)})
} else {
  console.log('could not connect')
}
