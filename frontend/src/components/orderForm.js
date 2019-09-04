import React, {useState} from 'react';
import Button from 'react-bootstrap/Button';
import {getProducts} from "../api/products";
import Total from "./total";
import "../styles/form.scss"


function Form() {

  const products = getProducts()

  let [selected, setSelected] = useState({})
  console.log(selected)

  function addItem(productId) {
    var newSelected = {...selected}
    if (selected[productId]) {
      newSelected[productId] += 1;
    } else {
      newSelected[productId] = 1;
    }
    setSelected(newSelected);
    console.log('addItem called', selected)
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
        {showSelected(selected, removeAllItem, removeOneOfItem)}
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

function showSelected(selected, removeAllItem, removeOneOfItem) {
  var result = []
  console.log("showSelected:",selected)
  for (var id in selected) {
    console.log(id)
    if (selected[id]) {
      result.push(selectedProduct(id, selected[id], removeAllItem, removeOneOfItem))
    }
  }
  console.log("showSelected called")
  return (
    <div className="flex-container">
      {result}
    </div>
  )
}

function selectedProduct(id, number, removeAllItem, removeOneOfItem) {
  return (
    <div className="selected-product">
      <div>
        {id}, {number}
      </div>
      <Button onClick={()=>{removeOneOfItem(id)}}>-</Button>
      <Button onClick={()=>{removeAllItem(id)}}>x</Button>
    </div>
  )
}

function getTotal(selected, products) {
  var tot = 0
  for (var product of products) {
    console.log("product:",product)
    console.log('number:', selected[product.id])
    tot -= + !!selected[product.id] && selected[product.id] * product.price
    console.log(tot)
  }
  console.log(tot)
  return tot
}

export default Form;
