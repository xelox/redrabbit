<script lang='ts'>
import {type TypeTask } from "../models/tasks";
import backend_adapter from "../util/backend_adapter";
import Checkbox from "./Checkbox.svelte";
import { slide } from 'svelte/transition';
import TaskCollection from "./TaskCollection.svelte";

export let node: TypeTask;

let subtascks_count: number;
$: {
  let f = (t: TypeTask, i: number = 0) => {
    for (const child of t.children.values()) i = f(child, i) + 1;
    return i;
  }
  subtascks_count = f(node);
}

const invoke_task_creation_wizzard = () => {
  const e = new CustomEvent('invoke_task_creation_wizzard', { detail: { parent: node, }});
  window.dispatchEvent(e);
}

const delete_task = () => {
  backend_adapter.tasks.delete({id: node.id}).then(()=>{
    const e = new CustomEvent(`remove_subtask:${node.parent_id??'root'}`, {detail: {task_id: node.id}});
    console.log(e)
    window.dispatchEvent(e);
  });
}
</script>

<main>
  <div class="body">
    <div class="left">
      <Checkbox done={node.done} started={node.started}/>
      <span class="name"> {node.name} </span>
    </div>
    <div class="right">
      <button class='interaction' on:click={invoke_task_creation_wizzard}><span>+</span></button>
      <button class='interaction' on:click={delete_task}><span>-</span></button>
    </div>
  </div>
  {#if node.children.size > 0}
    <button on:click={()=>{ node.is_open=!node.is_open}} class="children_divider"> 
      {subtascks_count} sub task{subtascks_count !== 1 ? 's' : ''}
    </button>
    {#if node.is_open}
      <div class="children_wrap" transition:slide>
        <TaskCollection collection={node.children} parent_id={node.id}/>
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

.interaction {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 22px;
  aspect-ratio: 1/1;
  border-radius: 100%;
}
</style>
