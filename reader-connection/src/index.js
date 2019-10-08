const serial = require('./serial')
const express = require('express')
const cors = require('cors')

const cardReader = serial.connect()
if (cardReader) {
  console.log('connected to serial Port')

  cardReader.write('test\n')
  cardReader.on('card', (card) => {console.log('Unhandled card:', card)})
  setupServer(cardReader)
} else {
  console.log('could not connect')
  process.exit(1);
}


var corsOptions = {
  origin: 'http://localhost:3000',
  optionsSuccessStatus: 200 // some legacy browsers (IE11, various SmartTVs) choke on 204
}


function setupServer(cardReader) {
  var app = express();

  app.get('/card', 
    cors(corsOptions),
    (req, res) => {
    cardReader.onOnce('card', (card) => {
      console.log('sending card:',card)
      res.send(card);
    })
  })
  app.listen(3001)
}
