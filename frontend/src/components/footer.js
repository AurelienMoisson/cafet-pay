import React from 'react';
import Button from 'react-bootstrap/Button';

import Total from './total';
import Validate from './validate';

import '../styles/footer.scss';

export default function Footer({selected, products, reductions}) {
  return (
    <div className='footer'>
      <Total selected={selected} products={products} reductions={reductions}/>
      <Validate selected={selected} products={products} reductions={reductions}/>
    </div>
  )
}
