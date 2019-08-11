const serial = require('./serial')
const express = require('express')

const cardReader = serial.connect()
if (cardReader) {
  console.log('connected to serial Port')

  cardReader.write('test\n')
  cardReader.on('card', (card) => {console.log('Card:', card)})
} else {
  console.log('could not connect')
}
