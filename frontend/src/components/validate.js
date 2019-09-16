import React, {useState} from 'react';
import Button from 'react-bootstrap/Button';
import Modal from 'react-awesome-modal';

import '../styles/validate.scss';

export default function Validate({selected, products, reductions, setSelected}) {

  const [waitingForCard, setWaitingForCard] = useState(false)

  let stopWaiting = () => {setSelected({}); setWaitingForCard(false)}
  let startWaiting = () => {setWaitingForCard(true)}

  return (
    <div className='validate'>
      <Button onClick={()=>{
        startWaiting();
        getCard(stopWaiting)
      }}>Valider</Button>
      <Modal
        visible={waitingForCard}
        effect='fadeInUp'
        height='300px'
        width='300px'
      >
        En attente de lecture de la carte...
      </Modal>
    </div>
  )
}

function getCard(stopWaiting) {
  setTimeout(stopWaiting, 1000)
}
