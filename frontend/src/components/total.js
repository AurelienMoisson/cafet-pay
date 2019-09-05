import React from 'react';
import '../styles/total.scss';

export default function Total({selected, products}) {
  const total = getTotal(selected, products)
  return (
    <div className='total'>
      Total : {formatTotal(total)}
    </div>
  )
}

function formatTotal(total) {
  let result = ' '
  if (total) {
    result = total>0?'+':''
  }
  let [euros, cents] = (Math.round(total*100)/100+'').split('.')
  cents = cents?cents:''
  cents = (cents.length===1)?cents+'0':cents
  result = cents?(euros+'.'+cents.substring(0,2)):euros
  result += '€'
  return result
}

function getTotal(selected, products) {
  var tot = 0
  for (var product of products) {
    tot -= + !!selected[product.id] && selected[product.id] * product.price
  }
  return tot
}
