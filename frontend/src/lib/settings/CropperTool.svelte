<script lang='ts'>
import { onMount } from "svelte";

export let aspect: {x: number, y:number};
export let round: boolean;
export let subject_src: string;

let width: number;
let height: number;
let min_width: number;
let min_height: number;
let axis: string;
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
    axis = 'width';

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
    axis = 'height';

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

onMount(()=>{
  main.addEventListener('mousedown', (e: Event) => {
    e.preventDefault();
    dragging = true;
  })
  window.addEventListener('mouseup', (e: Event) => {
    dragging = false;
  })
  window.addEventListener('mousemove', (e: MouseEvent) => {
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
  window.addEventListener('wheel', (e: WheelEvent) => {
    zoom(clamp(e.deltaY, -1, 1));
  })
})
</script>

<main bind:clientWidth={width} bind:clientHeight={height} bind:this={main}>
  <img src={subject_src} alt="" class='subject_img' style={`width: ${width}px; height: ${height}px; left: ${offset.x}px; top: ${offset.y}px`}>
  <div class="guideline" style="aspect-ratio: {aspect.x}/{aspect.y}; border-radius: {round ? '100vh':'4px'}"></div>
</main>

<style>
.guideline {
  border: 3px solid var(--text);
  z-index: 1;
}
.subject_img {
  position: absolute;
  top: 0;
  left: 0;
  z-index: -1;
  object-fit: cover;
  transition: 50ms linear;
}
main {
  position: fixed;
  left: 50%; top: 50%;
  transform: translate(-50%, -50%);
  width: 400px;
  height: 400px;
  /* overflow: hidden; */
}
</style>
