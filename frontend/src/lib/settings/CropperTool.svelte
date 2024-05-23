<script lang='ts'>
import { onMount } from "svelte";

export let output_res: {x: number, y:number};
const ratio = output_res.x / output_res.y; 
export let round: boolean;
export let subject_src: string;
export let on_submit: (src: string) => void;

let width: number;
let height: number;
let min_w: number;
let min_h: number;
let offset = { x: 0, y: 0 };
let min_offset = { x: 0, y: 0 };
let image_ratio: number;
let input_w: number;
let input_h: number;
const MAX_SIZE = 400;
let SIZE_X: number;
let SIZE_Y: number;
if (output_res.x < output_res.y) {
  SIZE_X = ratio * MAX_SIZE;
  SIZE_Y = MAX_SIZE
} else {
  SIZE_Y = MAX_SIZE / ratio;
  SIZE_X = MAX_SIZE;
}

const MIN_SIZE = Math.min(SIZE_X, SIZE_Y);

console.log(SIZE_X/SIZE_Y, output_res.x / output_res.y);

const WHITESPACE = 50;
let mask_style = `width: ${SIZE_X}px; height: ${SIZE_Y}px;`;
let guideline_style = `width: ${SIZE_X}px; height: ${SIZE_Y}px; top: ${WHITESPACE}px; left: ${WHITESPACE}px;`
if (round) {
  mask_style += `
    mask: radial-gradient(circle at center, transparent 0%, transparent ${MAX_SIZE / 2 - 0.5}px, rgba(0, 0, 0, 0.65) ${MAX_SIZE / 2 + 0.5}px, rgba(0, 0, 0, 0.65) 100%);
  `;
  guideline_style += 'border-radius: 100vh';
} else {
  mask_style += `
    box-shadow: inset 0 0 0 50px var(--crust); 
    background: transparent;
    opacity: 0.65;
  `;
}


const ZOOM_SPEED = 20;
let zoom: (dir: number) => void;
function post_zoom() {
    min_offset.y = SIZE_Y - height;
    min_offset.x = SIZE_X - width;
    offset.x = clamp(offset.x, min_offset.x, 0);
    offset.y = clamp(offset.y, min_offset.y, 0);
}

const image = new Image();
image.onload = function(e) {
  const target = e.target as HTMLImageElement;
  input_w = target.width;
  input_h = target.height;
  image_ratio = input_w / input_h;

  let magic: boolean;
  if (output_res.x === output_res.y) {
    magic = input_w > input_h;
  } else {
    magic = output_res.x < output_res.y;
  }
  if (magic) {
    height = MAX_SIZE;
    width = MAX_SIZE * image_ratio
    zoom = (dir: number) => {
      height = clamp(height - dir * ZOOM_SPEED, SIZE_Y, Infinity);
      width = height * image_ratio;
      post_zoom();
    }
  } else {
    height = MAX_SIZE / image_ratio;
    width = MAX_SIZE;
    zoom = (dir: number) => {
      width = clamp(width - dir * ZOOM_SPEED, SIZE_X, Infinity);  
      height = width / image_ratio;
      post_zoom();
    }
  }

  offset = { x: SIZE_X / 2 - width / 2, y: SIZE_Y / 2 - height / 2 };
  min_offset.y = SIZE_Y - height;
  min_offset.x = SIZE_X - width;
  min_w = width;
  min_h = height;
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

  const src_w = (SIZE_X / width) * input_w;
  const src_h = (SIZE_Y / height) * input_h;

  const src_offset_x = (-offset.x / SIZE_X) * src_w;
  const src_offset_y = (-offset.y / SIZE_Y) * src_h;

  ctx.drawImage(image, src_offset_x, src_offset_y, src_w, src_h, 0, 0, canvas.width, canvas.height);

  const output = canvas.toDataURL('image/png');
  on_submit(output);
}

onMount(()=>{
  ctx = canvas.getContext('2d')!;
  canvas.width = output_res.x;
  canvas.height = output_res.y;
  main.addEventListener('mousedown', (e: MouseEvent) => { 
    e.preventDefault();
    if(e.buttons === 1) {
      dragging = true; 
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
      <div>
        <button class='edit_btn'>Mirror V</button>
        <button class='edit_btn'>Mirror H</button>
      </div>
      <div>
        <button class='save_btn' on:click={save}>Save</button>
      </div>
    </div>
  </div>
<canvas bind:this={canvas}></canvas>
</main>

<style>
.subject_img {
  position: relative;
  top: 0;
  left: 0;
  z-index: -2;
  transition: 80ms linear;
}
.sub{
  z-index: 1;
}
.edit_wrap {
  overflow: hidden;
  position: relative;
  border-radius: 4px;
  background: var(--crust);
}
.sub {
  width: 100%;
  height: 100%;
  position: relative;
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
  display: flex;
  justify-content: space-between;
}
</style>
