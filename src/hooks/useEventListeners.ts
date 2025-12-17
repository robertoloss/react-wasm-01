import { useEffect } from "react";
import { event_listener } from '../../pkg/xonix/xonix';
import { event_listener as doom_listener } from "../../pkg/doom/doom";

type Game = 'xonix' | 'doom'

export default function useAddEventListeners(game: Game) {
  const listener = {
    doom: doom_listener,
    xonix: event_listener 
  }

  function handleKeyDown(e: KeyboardEvent) {
      e.preventDefault()
      listener[game](e.code) 
  }
  useEffect(()=>{
    document.addEventListener('keydown', handleKeyDown)
    return ()=>document.removeEventListener('keydown', handleKeyDown)
  },[])
}
