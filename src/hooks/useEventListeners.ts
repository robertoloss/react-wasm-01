import { useEffect } from "react";
import { event_listeners } from '../../pkg/react_wasm_01';

export default function useAddEventListeners() {

  function handleKeyDown(e: KeyboardEvent) {
      e.preventDefault()
      event_listeners(e.code) 
  }

  useEffect(()=>{
    document.addEventListener('keydown', handleKeyDown)

    return ()=>document.removeEventListener('keydown', handleKeyDown)
  },[])
}
