import React from 'react';
import Button from 'react-bootstrap/Button';

export default function ShowSelected({products, selected, removeAllItem, removeOneOfItem}) {
  var result = []
  for (var id in selected) {
    if (selected[id]) {
      result.push(selectedProduct(products.find((product)=>(product.id==id)), selected[id], removeAllItem, removeOneOfItem))
    }
  }
  return (
    <div className="flex-container">
      {result}
    </div>
  )
}

function selectedProduct(product, number, removeAllItem, removeOneOfItem) {
  return (
    <div className="selected-product">
      <div>
        {product.name} : {number}
      </div>
      <Button onClick={()=>{removeOneOfItem(product.id)}}>-</Button>
      <Button onClick={()=>{removeAllItem(product.id)}}>x</Button>
    </div>
  )
}
