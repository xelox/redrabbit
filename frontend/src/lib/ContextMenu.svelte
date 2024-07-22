<script lang='ts'>
import { onDestroy } from "svelte";
import type { TypeCtxMenu } from "../models/ctx_menu";
import xevents from "../util/xevents";


let active = false;
let items: TypeCtxMenu = []
let position = {x: 0, y: 0};

const listener = xevents.listen('open_context_menu', (req: {ctx_menu: TypeCtxMenu, position: {x: number, y: number}}) => {
  const fn = () => {
    active = true;
    position = req.position; 
    items = req.ctx_menu;
  }

  if (active) {
    active = false;
    setTimeout(fn, 100)
  } else fn();
})

onDestroy(()=>{
  listener.cleanup();
})

const use_callback = (callback: (() => void) | undefined) => {
  active = false;
  callback?.call(null);
}

document.addEventListener('click', e => {
  if (e.target !== main_dom) active = false;
})

document.addEventListener('contextmenu', e => {
  if (e.target !== main_dom) active = false;
})

let main_dom: HTMLElement | undefined;
</script>

<main class:open_ctx={active} style="top: {position.y}px; left: {position.x}px" bind:this={main_dom}>
  {#each items as item}
    {#if item.separator}
      <div class="separator"><span></span></div>
    {:else}
      <button on:click|stopPropagation={()=>use_callback(item.callback)}>
        <span class="hover_effect"></span>
        <span class="name"> {item.name} </span>
      </button>
    {/if}
  {/each}
</main>

<style>
.separator {
  padding: 8px 0;
}
.separator>span {
  display: block;
  border-bottom: 1px solid #555;
}
button {
  display: flex;
  padding: 8px 32px;
  background: #222;
  color: white;
  cursor: pointer;
  position: relative;
}
.name {
  position: relative;
}
button>.hover_effect {
  background: #333;
  width: 0%; height: 100%;
  position: absolute;
  top: 0; right: 0;
  transition: 200ms ease-in-out;
}
button:hover>.hover_effect {
  width: 100%;
}
main {
  z-index: -1;
  transition: 100ms;
  padding: 10px 0;
  position: fixed;
  background: #222;
  flex-direction: column;
  width: max-content;
  -webkit-box-shadow: -3px 3px 11px 0px rgba(0,0,0,0.17); 
  box-shadow: -3px 3px 11px 0px rgba(0,0,0,0.17);
  display: flex;
  transform: scale(0.8);
  opacity: 0;
  border: 1px solid #555;
  border-radius: 16px;
  overflow: hidden;
}
.open_ctx {
  opacity: 1;
  z-index: 1;
  transform: scale(1);
}
</style>
