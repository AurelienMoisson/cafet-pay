import React from 'react';

export default function Total({total}) {
  return (
    <div>
      {formatTotal(total)}
    </div>
  )
}

function formatTotal(total) {
  let result = ' '
  if (total) {
    result = total>0?'+':''
  }
  result += total + '$'
  return result
}
