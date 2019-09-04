import React from 'react';
import '../styles/total.scss';

export default function Total({total}) {
  return (
    <div className='total'>
      Total :
      {formatTotal(total)}
    </div>
  )
}

function formatTotal(total) {
  let result = ' '
  if (total) {
    result = total>0?'+':''
  }
  result += total + '€'
  return result
}
