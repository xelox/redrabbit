<script lang='ts'>
import type { NodeType } from "../models/tree";
import Checkbox from "./Checkbox.svelte";
import { slide } from 'svelte/transition';

export let node: NodeType;

let subtascks_count: number;
$: {
  let f = (t: NodeType, i: number = 0) => {
    for (const child of t.children) i = f(child, i) + 1;
    return i;
  }
  subtascks_count = f(node);
}
</script>

<main>
  <div class="body">
    <div class="left">
      <Checkbox done={node.done} started={node.started}/>
      <span class="name"> {node.name} </span>
    </div>
    <div class="right">
      <button>+</button>
    </div>
  </div>
  {#if node.children.length > 0}
  <button on:click={()=>{node.is_open=!node.is_open}} class="children_divider"> {subtascks_count} sub task{subtascks_count !== 1 ? 's' : ''} </button>
    {#if node.is_open}
    <div class="children_wrap" transition:slide>
      {#each node.children as child}
        <svelte:self node={child}/>
      {/each}
    </div>
    {/if}
  {/if}
</main>

<style>
.body, .left, .right {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
}

.body {
  color: black;
  background: #fafafa;
  padding: 0px 16px 8px 16px;
  justify-content: space-between;
}

.children_wrap {
  padding-left: 30px;
}

.children_divider {
  display: inline;
  text-align: left;
  border: none;
  padding: 2px 16px;
  width: calc(100% - 29px);
  margin-left: 30px;
  background: #edeff4;
  font-size: 0.76em;
  color: #555;
}

main {
  border-left: 1px solid #ddd;
  border-bottom: 1px solid #ddd;
  padding: 8px;
  background-color: #fafafa;
  border-bottom-left-radius: 5px;
  -webkit-box-shadow: -3px 3px 11px 0px rgba(0,0,0,0.17); 
  box-shadow: -3px 3px 11px 0px rgba(0,0,0,0.17);
}
</style>
