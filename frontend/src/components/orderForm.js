import React, {useState} from 'react';
import Button from 'react-bootstrap/Button';
import {getProducts} from "../api/products";
import Total from "./total";
import "../styles/form.scss"


function Form() {

  const products = getProducts()

  let [selected, setSelected] = useState({})
  console.log(selected)

  function addItem(product) {
    var newSelected = {...selected}
    if (selected[product.id]) {
      newSelected[product.id] += 1;
    } else {
      newSelected[product.id] = 1;
    }
    setSelected(newSelected);
    console.log('addItem called', selected)
  }

  return (
    <div className="form">
      <div className="flex-container">
        {getButtons(products, addItem)}
      </div>
      <div>
        {showSelected(selected)}
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
    <Button onClick={()=>{addItem(product)}} key={product.id}> {product.name} </Button>
  )
}

function showSelected(selected) {
  var result = []
  console.log("showSelected:",selected)
  for (var id in selected) {
    console.log(id)
    result.push(selectedProduct(id, selected[id]))
  }
  console.log("showSelected called")
  return result
}

function selectedProduct(id, number) {
  return (
    <div>
      <div>
        {id}, {number}
      </div>
      <Button onClick={()=>{}}/>
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
