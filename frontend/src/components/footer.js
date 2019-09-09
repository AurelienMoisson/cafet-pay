import React from 'react';
import Button from 'react-bootstrap/Button';

import Total from './total';

import '../styles/footer.scss';

export default function Footer({selected, products, reductions}) {
  return (
    <div className='footer'>
      <Total selected={selected} products={products} reductions={reductions}/>
      <Button onClick={()=>{}}>Valider</Button>
    </div>
  )
}
