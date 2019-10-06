export async function getCard() {
  let result = fetch('http://localhost:3001/card')
    .then((data) => {return data.results})
  let r = await result;
  console.log(r);
  return r;
}
