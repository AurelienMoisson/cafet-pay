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
  return new CardReader(new SerialPort(portPath, {baudRate:9600}))
}

class CardReader {
  constructor(device) {
    console.debug('making CardReader')
    this.device = device
    device.on('data', this.parse)
    this.onFunctions = {'card':null}
    this.previousCards = []
    this.buffer = ""

    this.write = (...args) => this.device.write(...args)
  }

  parse(data) {
    console.debug('parsing data')
    var received = ''+data
    for (var i = 0; i<received.length; i++){
      if (received[i] == '\n') {
        sendCard()
      } else {
        this.buffer += received[i]
      }
    }
  }

  sendCard() {
    console.debug('sending card')
    if (this.onFunctions.card) {
      this.onFunctions.card(this.buffer)
    } else {
      this.previousCards.push(this.buffer)
    }
    this.buffer = ""
  }

  on(label, func) {
    this.onFunctions[label] = func
    if (this.onFunctions.card) {
      for (var i = 0; i<this.previousCards.length; i++) {
        this.onFunctions.card(this.previousCards[i])
      }
      this.previousCards = []
    }
  }
}


module.exports = {connect}
