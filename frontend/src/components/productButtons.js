import React from 'react';
import Button from 'react-bootstrap/Button';

export default function GetProductForm({products, addItem}) {
  let results = []
  let categories = {}
  for (var product of products) {
    if (categories[product.category]) {
      categories[product.category].push(product)
    } else {
      categories[product.category] = [product]
    }
  }
  for (var category in categories) {
    results.push(getCategoryForm(category, categories[category], addItem))
  }
  return (
    <div>
      {results}
    </div>
  )
}

function getCategoryForm(categoryName, products, addItem) {
  let results = [];
  for (var product of products) {
    results.push(IndividualButton(product, addItem))
  }
  return (
    <div className="category flex-container">
      <span className="category-name">{categoryName}</span>
      <div className="category-choices">
        {results}
      </div>
    </div>
  );
}

function IndividualButton(product, addItem) {
  return (
    <Button onClick={()=>{addItem(product.id)}} key={product.id}> {product.name} </Button>
  )
}
