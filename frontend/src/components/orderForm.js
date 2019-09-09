import React, {useState} from 'react';

import {getProducts} from "../api/products";
import ShowSelected from './selected';
import GetProductForm from './productButtons';
import Reductions, {updatePossibleReductions} from './reductions';
import Total from "./total";

import "../styles/form.scss"


function Form() {

  const products = getProducts()

  let [selected, setSelected] = useState({})
  let [appliedReductions, setAppliedReductions] = useState({})

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

  function updateReduction(reduction, maxTimesApplied) {
    if (!appliedReductions[reduction.id]) {
      if (maxTimesApplied) {
        let newAppliedReductions = {...appliedReductions}
        newAppliedReductions[reduction.id] = {
          reduction:reduction,
          timesApplied:maxTimesApplied,
          maxTimesApplied,
        }
        setAppliedReductions(newAppliedReductions)
        return;
      }
    } else if (appliedReductions[reduction.id].maxTimesApplied!==maxTimesApplied) {
      let newAppliedReductions = {...appliedReductions}
      let newReduction = {...appliedReductions[reduction.id]}
      newReduction.timesApplied = maxTimesApplied + newReduction.timesApplied - newReduction.maxTimesApplied
      newReduction.timesApplied = (newReduction.timesApplied<0)?0:newReduction.timesApplied
      newReduction.maxTimesApplied = maxTimesApplied
      newAppliedReductions[reduction.id] = newReduction
      setAppliedReductions(newAppliedReductions)
      return;
    }
  }

  function removeOneReduction(reductionId) {
    if (appliedReductions[reductionId] && appliedReductions[reductionId].timesApplied>0) {
      let newAppliedReductions = {...appliedReductions}
      let newReduction = {...appliedReductions[reductionId]}
      newReduction.timesApplied -= 1
      newAppliedReductions[reductionId] = newReduction
      setAppliedReductions(newAppliedReductions)
    }
  }

  function addOneReduction(reductionId) {
    if (appliedReductions[reductionId] && appliedReductions[reductionId].timesApplied<appliedReductions[reductionId].maxTimesApplied) {
      let newAppliedReductions = {...appliedReductions}
      let newReduction = {...appliedReductions[reductionId]}
      newReduction.timesApplied += 1
      newAppliedReductions[reductionId] = newReduction
      setAppliedReductions(newAppliedReductions)
    }
  }

  updatePossibleReductions({selected, products, updateReduction})
  return (
    <div className="form">
      <GetProductForm products={products} addItem={addItem}/>
      <ShowSelected products={products} selected={selected} removeAllItem={removeAllItem} removeOneOfItem={removeOneOfItem}/>
      <Reductions appliedReductions={appliedReductions} removeOneReduction={removeOneReduction} addOneReduction={addOneReduction}/>
      <Total selected={selected} products={products} reductions={appliedReductions}/>
    </div>
  )
}



export default Form;
