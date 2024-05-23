<script lang='ts'>
import { onMount } from "svelte";

export let aspect: {x: number, y:number};
const ratio = aspect.x / aspect.y; 
export let round: boolean;
export let subject_src: string;

let width: number;
let height: number;
let min_width: number;
let offset = { x: 0, y: 0 };
let min_offset = { x: 0, y: 0 };
let image_ratio: number;
const MAX_SIZE = 400;
let SIZE_X: number;
let SIZE_Y: number;
if (aspect.x < aspect.y) {
  SIZE_X = ratio * MAX_SIZE;
  SIZE_Y = MAX_SIZE
} else {
  SIZE_Y = MAX_SIZE / ratio;
  SIZE_X = MAX_SIZE;
}

const WHITESPACE = 50;
let mask_style = `width: ${SIZE_X}px; height: ${SIZE_Y}px;`;
let guideline_style = `width: ${SIZE_X}px; height: ${SIZE_Y}px; top: ${WHITESPACE}px; left: ${WHITESPACE}px;`
if (round) {
  mask_style += `
    mask: radial-gradient(circle at center, transparent 0%, transparent ${MAX_SIZE / 2 - 0.5}px, rgba(0, 0, 0, 0.5) ${MAX_SIZE / 2 + 0.5}px, rgba(0, 0, 0, 0.5) 100%);
  `;
  guideline_style += 'border-radius: 100vh';
} else {
  mask_style += `
    box-shadow: inset 0 0 0 50px var(--crust); 
    background: transparent;
    opacity: 0.5;
  `;
}


console.log(mask_style);
const ZOOM_SPEED = 20;

function zoom(dir: number) {
  width = clamp(width - dir * ZOOM_SPEED, min_width, Infinity);  
  height = width / image_ratio;
  min_offset.y = SIZE_Y - height;
  min_offset.x = SIZE_X - width;
  offset.x = clamp(offset.x, min_offset.x, 0);
  offset.y = clamp(offset.y, min_offset.y, 0);
}

const image = new Image();
image.onload = function(e) {
  const target = e.target as HTMLImageElement;
  const w = target.width;
  const h = target.height;
  image_ratio = w / h;

  if (SIZE_X < SIZE_Y) {
    width = MAX_SIZE * image_ratio;
    height = MAX_SIZE;
    offset = { x: 0, y: MAX_SIZE / 2 - height / 2};
  } else {
    height = MAX_SIZE / image_ratio;
    width = MAX_SIZE;
    offset = { x: MAX_SIZE / 2 - width / 2, y: 0 };
  }
  min_offset.y = SIZE_Y - height;
  min_offset.x = SIZE_X - width;
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
  canvas.width = MAX_SIZE;
  canvas.height = MAX_SIZE;
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
  <div class="main">
    <h1>Crop Image</h1>
    <div class="edit_wrap" style={`width: ${SIZE_X}px; height: ${SIZE_Y}px; padding: ${WHITESPACE}px;`}>
      <div class="sub">
        <img src={subject_src} alt="" class='subject_img' style={`width: ${width}px; height: ${height}px; left: ${offset.x}px; top: ${offset.y}px`}>
      </div>
      <div class="mask" style={mask_style}></div>
      <div class="guideline" style={guideline_style}></div>
    </div>
    <div class="footer">
      <button class='edit_btn'>Mirror V</button>
      <button class='edit_btn'>Mirror H</button>
      <button class='save_btn'>Save</button>
    </div>
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
  z-index: 1;
}
.edit_wrap {
  overflow: hidden;
  position: relative;
  border-radius: 10px;
  background: var(--crust);
}
.sub {
  width: 100%;
  height: 100%;
  position: relative;
  border-radius: 4px;
}
.mask{
  padding: inherit;
  top: 0; left: 0;
  position: absolute;
  background: var(--crust);
  pointer-events: none;
  z-index: 2;
}
.guideline {
  position: absolute;
  border: 1px solid var(--crust);
  transform: translate(-1px, -1px);
  z-index: 9;
}
canvas {
  display: none;
}
button {
  padding: 4px 10px 4px 10px;
  border-radius: 2px;
  font-family: 'Noto Sans';
  background: var(--btn-blue);
  color: white;
  -webkit-box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24); 
  box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24);
}
.save_btn {
  background: var(--btn-green);
}
.main {
  padding: 20px;
  position: absolute;
  background: var(--surface0);
  width: max-content;
  height: max-content;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  border-radius: 4px;
  cursor: move;
  -webkit-box-shadow: 0px 33px 19px 2px rgba(0,0,0,0.2), 0px 24px 9px -2px rgba(0,0,0,0.17), 0px 9px 11px -2px rgba(0,0,0,0.58); 
  box-shadow: 0px 33px 19px 2px rgba(0,0,0,0.2), 0px 24px 9px -2px rgba(0,0,0,0.17), 0px 9px 11px -2px rgba(0,0,0,0.58);
}
main {
  position: fixed;
  top: 0; left: 0;
  width: 100vw; height: 100vh;
  background: rgba(0, 0, 0, 0.4);
}
h1 {
  margin-bottom: 20px;
}
.footer {
  margin-top: 20px;
}
</style>
