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
    device.on('data', (data) => this.parse(data))
    this.onFunctions = {'card':null}
    this.onOnceFunctions = {'card':[]}
    this.buffer = ""

    this.write = (...args) => this.device.write(...args)
  }

  parse(data) {
    var received = ''+data
    for (var i = 0; i<received.length; i++){
      if (received[i] === '\n') {
        this.sendCard()
      } else {
        this.buffer += received[i]
      }
    }
  }

  sendCard() {
    if (this.onOnceFunctions.card && this.onOnceFunctions.card.length) {
      console.log(this.onOnceFunctions.card)
      let func = this.onOnceFunctions.card.shift()
      func(this.buffer)
    } else if (this.onFunctions.card) {
      this.onFunctions.card(this.buffer)
    }
    this.buffer = ""
  }

  on(label, func) {
    this.onFunctions[label] = func
  }

  onOnce(label, func) {
    if (this.onOnceFunctions[label]) {
      this.onOnceFunctions[label].push(func)
    } else {
      this.onOnceFunctions[label] = [func]
    }
  }

}


module.exports = {connect}
