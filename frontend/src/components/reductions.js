import React from 'react';
import Button from 'react-bootstrap/Button';

import {getActiveReductions} from '../accountsAPI/reductions';
import {getTotalSpent, formatTotal} from './total';

import '../styles/reductions.scss'

export default function Reductions({appliedReductions, removeOneReduction, addOneReduction}) {

  var reductionsObjects = []

  console.log('appliedReductions', appliedReductions)
  for (var reductionId in appliedReductions) {
    console.log(reductionId)
    const reduction = appliedReductions[reductionId]
    if (reduction.maxTimesApplied) {
      reductionsObjects.push(SingleReduction(reduction.reduction, reduction.maxTimesApplied, reduction.timesApplied, removeOneReduction, addOneReduction))
    }
  }
  console.log('after loop')

  if (reductionsObjects.length) {
    return (
      <div>
        <div className='category-name'>
          reductions:
        </div>
        <div className='reductions-container'>
          {reductionsObjects}
        </div>
      </div>
    )
  }
  return (
    <div/>
  )
}

export function updatePossibleReductions({selected, products, updateReduction}) {
  const reductions = getActiveReductions();
  const spent = getTotalSpent(selected, products)
  for (var reduction of reductions) {
    var productsNotUsed = {...selected}
    var maxTimesApplied = -1
    var canApply = true
    while (canApply) {
      maxTimesApplied += 1
      for (var possibleProducts of reduction.productsBought) {
        canApply = canApply && removeOne(productsNotUsed, possibleProducts)
      }
    }
    updateReduction(reduction, maxTimesApplied)
  }
}


function removeOne(selected, possibleProducts) {
  for (var product of possibleProducts) {
    if (selected[product] && selected[product]>0) {
      selected[product] -= 1
      return true
    }
  }
  return false
}

function SingleReduction(reduction, maxTimes, times, removeOneReduction, addOneReduction) {
  return (
    <div className='reduction'>
      <div className='reduction-header'>
        <div className='reduction-name'>
          {reduction.name} 
        </div>
        <div className='reduction-price'>
          {formatTotal(reduction.price)}
        </div>
      </div>
      <div className='reduction-content'>
        <div className='reduction-description'>
          {reduction.description}
        </div>
      </div>
      <div className='reduction-footer'>
        <div className='reduction-times'>
          {times}/{maxTimes}
        </div>
        <div className='reduction-total'>
          total:{' '}
          {formatTotal(times*reduction.price)}
        </div>
        <Button onClick={()=>removeOneReduction(reduction.id)}>-</Button>
        <Button onClick={()=>addOneReduction(reduction.id)}>+</Button>
      </div>
    </div>
  )
}
