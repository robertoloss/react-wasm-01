import { useEffect, useState } from 'react'
import init from '../pkg/react_wasm_01'
import RootComponent from './RootComponent'

function App() {
  const [ startApp, setStartApp ] = useState(false)
  
  useEffect(()=>{
    init().then(()=>setStartApp(true))
  },[])

  if (!startApp) return <></>

  return (
    <RootComponent />
  )
}

export default App
