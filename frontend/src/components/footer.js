import React from 'react';
import Button from 'react-bootstrap/Button';

import Total from './total';
import Validate from './validate';
import Cancel from './cancel';

import '../styles/footer.scss';

export default function Footer({selected, products, reductions, setSelected}) {
  return (
    <div className='footer'>
      <Total selected={selected} products={products} reductions={reductions}/>
      <Validate selected={selected} products={products} reductions={reductions} setSelected={setSelected}/>
      <Cancel setSelected={setSelected}/>
    </div>
  )
}
