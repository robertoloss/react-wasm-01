import { useEffect } from "react";
import { event_listener } from '../../pkg/react_wasm_01';

export default function useAddEventListeners() {

  function handleKeyDown(e: KeyboardEvent) {
      e.preventDefault()
      event_listener(e.code) 
  }

  useEffect(()=>{
    document.addEventListener('keydown', handleKeyDown)

    return ()=>document.removeEventListener('keydown', handleKeyDown)
  },[])
}
