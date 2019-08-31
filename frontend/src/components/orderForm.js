import React, {useState} from 'react';
import Button from 'react-bootstrap/Button';
import products from "../products";
import "../styles/form.scss"

function Form() {
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
        {getButtons(addItem)}
      </div>
      <div>
        {showSelected(selected)}
      </div>
    </div>
  )
}

function getButtons(addItem) {
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

export default Form;
