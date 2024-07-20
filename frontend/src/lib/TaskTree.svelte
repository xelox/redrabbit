<script lang='ts'>
import {type TypeTask } from "../models/tasks";
import backend_adapter from "../util/backend_adapter";
import Checkbox from "./Checkbox.svelte";
import TaskCollection from "./TaskCollection.svelte";
import xevents from "../util/xevents";
import { onMount } from "svelte";
    import type { TypeCtxMenu } from "../models/ctx_menu";

export let task: TypeTask;
let subtask_count: number;

const recount_tasks = () => {
  console.log('updating count', '"', task.name, '"');
  for (const child of task.children.values()) {
    child.parent_recount = recount_tasks;
  }
  let f = (t: TypeTask, i: number = 0) => {
    for (const child of t.children.values()) {
      i = f(child, i) + 1;
    }
    return i;
  }
  subtask_count = f(task);
  task.parent_recount?.call(null);
}

onMount(()=>{
  recount_tasks();
})

const invoke_task_creation_wizzard = () => {
  xevents.emit('invoke_task_creation_wizzard', task);
}

const delete_task = () => {
  backend_adapter.tasks.delete({id: task.id}).then(() => {
    xevents.emit(`remove_task:${task.parent_id??'root'}`, task.id);
  })
}

const expand_toggle = () => {
  // backend_adapter.tasks.expand(!task.is_open, 'self')
  task.is_open = !task.is_open;
}

const open_context_menu = (e: MouseEvent) => {
  e.stopPropagation();
  const ctx_menu: TypeCtxMenu = [
    {
      name: "Create Subtask",
      callback: invoke_task_creation_wizzard,
    },
    {
      name: "Delete",
      callback: delete_task,
    },
    {
      name: "Pin",
    },
    {
      separator: true, 
    },
    {
      name: "Expand This",
    },
    {
      name: "Expand All",
    },
  ];
  const position = {x: e.clientX, y: e.clientY};

  xevents.emit('open_context_menu', {ctx_menu, position})
}

</script>

<main on:contextmenu|preventDefault={open_context_menu}>
  <div class="body">
    <Checkbox done={task.done} started={task.started}>{task.name}</Checkbox>
    <div class="right">
      <button class='interaction' on:click={invoke_task_creation_wizzard}><span>+</span></button>
      <button class='interaction' on:click={delete_task}><span>-</span></button>
    </div>
  </div>
  {#if task.children.size > 0}
    <button on:click={expand_toggle} class="children_divider"> 
      {subtask_count} sub task{subtask_count !== 1 ? 's' : ''}
    </button>
  {/if}
  <TaskCollection {recount_tasks} parent={task}/>
</main>

<style>
.body, .right {
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
