import React from 'react';
import Button from 'react-bootstrap/Button';

export default function Cancel({setSelected}) {
  return (
    <div className='cancel'>
      <Button onClick={()=>{setSelected({})}}>
        Annuler
      </Button>
    </div>
  )
}
