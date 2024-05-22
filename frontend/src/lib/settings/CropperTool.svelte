<script lang='ts'>
import { onMount } from "svelte";

export let aspect: {x: number, y:number};
export let round: boolean;
export let subject_src: string;

let width: number;
let height: number;
let min_width: number;
let min_height: number;
let offset = { x: 0, y: 0 };
let min_offset = { x: 0, y: 0 };
let ratio: number;
const SIZE = 400;
const ZOOM_SPEED = 10;
let zoom: (dir: number) => void;

const image = new Image();
image.onload = function(e) {
  const target = e.target as HTMLImageElement;
  const w = target.width;
  const h = target.height;
  ratio = w / h;

  if (w < h) {
    height = SIZE / ratio;
    width = SIZE;

    offset = { x: 0, y: SIZE / 2 - height / 2};
    min_offset.y = SIZE - height;

    zoom = (dir) => {
      width = clamp(width - dir * ZOOM_SPEED, min_width, Number.MAX_VALUE);  
      height = width * ratio;
      min_offset.y = SIZE - height;
      min_offset.x = SIZE - width;
      offset.x = clamp(offset.x, min_offset.x, 0);
      offset.y = clamp(offset.y, min_offset.y, 0);
    }
  } else {
    width = ratio * SIZE;
    height = SIZE;

    offset = { x: SIZE / 2 - width / 2, y: 0 };
    min_offset.x = SIZE - width;

    zoom = (dir) => {
      height = clamp(height - dir * ZOOM_SPEED, min_height, Number.MAX_VALUE);  
      width = height * ratio;
      min_offset.x = SIZE - width;
      min_offset.y = SIZE - height;
      offset.x = clamp(offset.x, min_offset.x, 0);
      offset.y = clamp(offset.y, min_offset.y, 0);
    }
  }
  min_width = width;
  min_height = height;
}
image.src = subject_src;

let main: HTMLElement;
let dragging = false;
let last_m_x = 0;
let last_m_y = 0;

function clamp(val: number, min: number, max: number):number {
  if (val > max) return max;
  if (val < min) return min;
  return val;
}

let canvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D;
function save(){
  ctx.clearRect(0, 0, canvas.width, canvas.height); 
  ctx.drawImage(image, offset.x, offset.y, width, height);
}

onMount(()=>{
  ctx = canvas.getContext('2d')!;
  canvas.width = SIZE;
  canvas.height = SIZE;
  main.addEventListener('mousedown', (e: MouseEvent) => { 
    e.preventDefault();
    if(e.buttons === 1) {
      dragging = true; 
    }
    else if (e.buttons === 2 ) {
      save();
    }
  })
  window.addEventListener('mouseup', () => { dragging = false; })
  main.addEventListener('mousemove', (e: MouseEvent) => {
    const new_x = e.x;
    const new_y = e.y;
    const delta_x = last_m_x - new_x;
    const delta_y = last_m_y - new_y;
    last_m_x = new_x;
    last_m_y = new_y;
    if (dragging) {
      offset.x = clamp(offset.x - delta_x, min_offset.x, 0);
      offset.y = clamp(offset.y - delta_y, min_offset.y, 0);
    }
  })
  main.addEventListener('wheel', (e: WheelEvent) => {
    zoom(clamp(e.deltaY, -1, 1));
  })
})
</script>

<main bind:this={main}>
  <h1>Crop Image</h1>
  <div class="sub">
    <img src={subject_src} alt="" class='subject_img' style={`width: ${width}px; height: ${height}px; left: ${offset.x}px; top: ${offset.y}px`}>
    <div class="mask"></div>
  </div>
</main>
<canvas bind:this={canvas}></canvas>

<style>
.subject_img {
  position: relative;
  top: 0;
  left: 0;
  z-index: -2;
  object-fit: cover;
  transition: 80ms linear;
}
.sub{
  cursor: move;
  z-index: 1;
}
.sub {
  position: relative;
  width: 400px;
  height: 400px;
  left: 50%;
  transform: translateX(-50%);
  overflow: hidden;
  border-radius: 4px;
}
.mask {
  position: absolute;
  width: inherit;
  height: inherit;
  background: black;
  mask: radial-gradient(circle at center, transparent 0%, transparent 200px, rgba(0, 0, 0, 0.5) 201px, rgba(0, 0, 0, 0.5) 100%);
  pointer-events: none;
  z-index: 2;
}
canvas {
  display: none;
}
main {
  position: fixed;
  background: var(--crust);
  width: 600px;
  height: 750px;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  border-radius: 4px;
}
</style>
