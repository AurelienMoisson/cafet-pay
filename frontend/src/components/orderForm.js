import React, {useState} from 'react';

import {getProducts} from "../api/products";
import ShowSelected from './selected';
import GetProductForm from './productButtons';
import Total from "./total";

import "../styles/form.scss"


function Form() {

  const products = getProducts()

  let [selected, setSelected] = useState({})

  function addItem(productId) {
    var newSelected = {...selected}
    if (selected[productId]) {
      newSelected[productId] += 1;
    } else {
      newSelected[productId] = 1;
    }
    setSelected(newSelected);
  }

  function removeAllItem(productId) {
    var newSelected = {...selected}
    if (selected[productId]) {
      newSelected[productId] = 0;
      setSelected(newSelected);
    }
  }

  function removeOneOfItem(productId) {
    var newSelected = {...selected}
    if (selected[productId]) {
      newSelected[productId] = newSelected[productId] - (newSelected[productId] > 0);
      setSelected(newSelected);
    }
  }

  return (
    <div className="form">
        <GetProductForm products={products} addItem={addItem}/>
      <div>
        <ShowSelected products={products} selected={selected} removeAllItem={removeAllItem} removeOneOfItem={removeOneOfItem}/>
      </div>
      <Total selected={selected} products={products}/>
    </div>
  )
}



export default Form;
