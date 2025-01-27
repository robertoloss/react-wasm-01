import { useEffect, useRef } from 'react';
import { render_game } from '../pkg/react_wasm_01';

export default function RootComponent() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const animationFrameId = useRef<number | null>(null);
  const ctxRef = useRef<CanvasRenderingContext2D | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctxRef.current = ctx;

    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
    ctx.imageSmoothingEnabled = false;

    function gameLoop() {
      if (!ctx || !canvas) return;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      render_game(ctx)
      animationFrameId.current = requestAnimationFrame(gameLoop);
    };
    gameLoop()
    return () => {
      if (animationFrameId.current) {
        cancelAnimationFrame(animationFrameId.current);
      }
    };
  }, []);

  
  return (
    <div className='flex flex-col h-screen w-screen bg-zinc-700 items-center justify-center text-white'>
      <div className='flex relative w-full h-full flex-col max-w-[800px] max-h-[600px] bg-blue-800'>
        <div className='flex absolute top-0 left-0 w-full h-full bg-orange-100'/>
        <canvas
          ref={canvasRef}
          className='flex absolute top-0 left-0 w-full h-full'
        />
      </div>
    </div>
  )
}
