import React, {useState} from 'react';
import Button from 'react-bootstrap/Button';
import {getProducts} from "../api/products";
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
      <div className="flex-container">
        {getButtons(products, addItem)}
      </div>
      <div>
        {showSelected(products, selected, removeAllItem, removeOneOfItem)}
      </div>
      <Total total={getTotal(selected, products)}/>
    </div>
  )
}

function getButtons(products, addItem) {
  let results = [];
  for (var product of products) {
    results.push(getIndividualButton(product, addItem))
  }
  return results;
}

function getIndividualButton(product, addItem) {
  return (
    <Button onClick={()=>{addItem(product.id)}} key={product.id}> {product.name} </Button>
  )
}

function showSelected(products, selected, removeAllItem, removeOneOfItem) {
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

function getTotal(selected, products) {
  var tot = 0
  for (var product of products) {
    tot -= + !!selected[product.id] && selected[product.id] * product.price
  }
  return tot
}

export default Form;
