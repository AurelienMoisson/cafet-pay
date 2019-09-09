export function getActiveReductions() {
  return [
    {
      name:'3/3 sandwich',
      description:"ne s'applique que sur les paires de sandwiches 1/3 et 2/3 ayant la même recette",
      minimumSpent:0,
      // [[1,2], [3]] means (1 or 2) and 3
      // things like [[1,2], [2,3]] are not supported (yet)
      productsBought:[[1], [3]],
      price:-1,
    }
  ]
}
