const serial = require('./serial')
const express = require('express')

const cardReader = serial.connect()
if (cardReader) {
  console.log('connected to serial Port')

  cardReader.write('test\n')
  cardReader.on('card', (card) => {console.log('Unhandled card:', card)})
  setupServer(cardReader)
} else {
  console.log('could not connect')
}

function setupServer(cardReader) {
  var app = express();

  app.get('/card', (req, res) => {
    cardReader.onOnce('card', (card) => {
      res.send(card);
    })
  })
  app.listen(3001)
}
