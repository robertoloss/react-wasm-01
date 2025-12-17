import { createBrowserRouter, RouterProvider } from 'react-router'
import Xonix from './Xonix'
import Doom from './Doom'

const router = createBrowserRouter([
  {
    path: 'xonix',
    element: <Xonix/>
  },
  {
    path: 'doom',
    element: <Doom/>
  }
])

function App() {

  return (
    <RouterProvider router={router} />
  )
}

export default App
