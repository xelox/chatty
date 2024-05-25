<script lang='ts'>
import { onMount } from "svelte";
import Button from "../components/Button.svelte";

// Component Inputs.
export let output_res: {x: number, y:number};
export let round: boolean;
export let input_src: string;
export let on_submit: (output: {url: string, blob: Blob}) => void;

const ratio = output_res.x / output_res.y; 

// Tool state.
let preview_w: number;
let preview_h: number;
let preview_offset = { x: 0, y: 0 };
let preview_offset_limit = { x: 0, y: 0 };

// Source Image Properties.
let image_ratio: number;
let input_w: number;
let input_h: number;

// Tool setup.
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

// Utility functions
function clamp(val: number, min: number, max: number):number {
  if (val > max) return max;
  if (val < min) return min;
  return val;
}

// Constant style setup.
const WHITESPACE = 50;
const make_style = () => {
  let MASK_STYLE = `width: ${SIZE_X}px; height: ${SIZE_Y}px;`;
  let GUIDELINE_STYLE = `width: ${SIZE_X}px; height: ${SIZE_Y}px; top: ${WHITESPACE}px; left: ${WHITESPACE}px;`
  if (round) {
    MASK_STYLE += `mask: radial-gradient(circle at center, transparent 0%, transparent ${MAX_SIZE / 2 - 0.5}px, rgba(0, 0, 0, 0.65) ${MAX_SIZE / 2 + 0.5}px, rgba(0, 0, 0, 0.65) 100%);`;
    GUIDELINE_STYLE += 'border-radius: 100vh';
  } else {
    MASK_STYLE += `box-shadow: inset 0 0 0 50px var(--crust); background: transparent; opacity: 0.65;`;
  }
  const EDIT_WRAP_STYLE = `width: ${SIZE_X}px; height: ${SIZE_Y}px; padding: ${WHITESPACE}px;`;
  return {MASK_STYLE, GUIDELINE_STYLE, EDIT_WRAP_STYLE};
}

const {MASK_STYLE, GUIDELINE_STYLE, EDIT_WRAP_STYLE} = make_style();


// Zoom declaration.
const ZOOM_SPEED = 25;
let zoom: (dir: number) => void;

// Loading image properies to then setup initial tool state.
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
    preview_h = MAX_SIZE;
    preview_w = MAX_SIZE * image_ratio
    zoom = (dir: number) => {
      preview_h = clamp(preview_h - dir * ZOOM_SPEED, SIZE_Y, Infinity);
      preview_w = preview_h * image_ratio;
    }
  } else {
    preview_h = MAX_SIZE / image_ratio;
    preview_w = MAX_SIZE;
    zoom = (dir: number) => {
      preview_w = clamp(preview_w - dir * ZOOM_SPEED, SIZE_X, Infinity);  
      preview_h = preview_w / image_ratio;
    }
  }

  preview_offset = { x: SIZE_X / 2 - preview_w / 2, y: SIZE_Y / 2 - preview_h / 2 };
  preview_offset_limit.y = SIZE_Y - preview_h;
  preview_offset_limit.x = SIZE_X - preview_w;
}
image.src = input_src;

// Drawing to canvas and submitting the cropped image.
let canvas: HTMLCanvasElement;
function save(){
  const ctx = canvas.getContext('2d');
  // WARNING: This error handle is not acceptable for prod.
  if (!ctx) return console.error('Operation not supported by the browser.');
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  const src_w = (SIZE_X / preview_w) * input_w;
  const src_h = (SIZE_Y / preview_h) * input_h;

  const src_offset_x = (-preview_offset.x / SIZE_X) * src_w;
  const src_offset_y = (-preview_offset.y / SIZE_Y) * src_h;

  ctx.drawImage(image, src_offset_x, src_offset_y, src_w, src_h, 0, 0, canvas.width, canvas.height);

  const url = canvas.toDataURL('image/png');
  canvas.toBlob((blob) => {
    // WARNING: This error handle is not acceptable for prod.
    if (!blob) return console.error("Failed to create blob.");
    on_submit({url, blob});
  }, 'image/png', 1)
}

// Handeling input events.
let main: HTMLElement;
let dragging = false;
const mouse = { x: 0, y: 0 };
onMount(()=>{
  canvas.width = output_res.x;
  canvas.height = output_res.y;
  main.addEventListener('mousedown', (e: MouseEvent) => { 
    e.preventDefault();
    if(e.buttons === 1) {
      dragging = true; 
    }
  })
  main.addEventListener('mouseup', () => { dragging = false; })
  main.addEventListener('mousemove', (e: MouseEvent) => {
    // Mouse dragging logic.
    if (dragging) {
      const delta_x = mouse.x - e.x;
      const delta_y = mouse.y - e.y;
      preview_offset.x = clamp(preview_offset.x - delta_x, preview_offset_limit.x, 0);
      preview_offset.y = clamp(preview_offset.y - delta_y, preview_offset_limit.y, 0);
    }
    mouse.x = e.x;
    mouse.y = e.y;
  })
  main.addEventListener('wheel', (e: WheelEvent) => {
    zoom(clamp(e.deltaY, -1, 1));

    // Post zoom
    preview_offset_limit.y = SIZE_Y - preview_h;
    preview_offset_limit.x = SIZE_X - preview_w;
    preview_offset.x = clamp(preview_offset.x, preview_offset_limit.x, 0);
    preview_offset.y = clamp(preview_offset.y, preview_offset_limit.y, 0);
  })
})
</script>

<main bind:this={main}>
  <div class="arc">
    <h1>Crop Image</h1>
    <div class="edit_wrap" style={EDIT_WRAP_STYLE}>
      <div class="sub">
        <img src={input_src} alt="" class='subject_img' style={`width: ${preview_w}px; height: ${preview_h}px; left: ${preview_offset.x}px; top: ${preview_offset.y}px`}>
      </div>
      <div class="mask" style={MASK_STYLE}></div>
      <div class="guideline" style={GUIDELINE_STYLE}></div>
    </div>
    <div class="footer">
      <div>
        <Button>Mirror V</Button>
        <Button>Mirror H</Button>
      </div>
      <div>
        <Button action={save} bg={'green'}>Save</Button>
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
  cursor: move;
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
.arc {
  padding: 20px;
  position: absolute;
  background: var(--surface0);
  width: max-content;
  height: max-content;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  border-radius: 4px;
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
