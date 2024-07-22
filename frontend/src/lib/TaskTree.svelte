<script lang='ts'>
import {compare_completion, type TypeTask } from "../models/tasks";
import backend_adapter from "../util/backend_adapter";
import Checkbox from "./Checkbox.svelte";
import TaskCollection from "./TaskCollection.svelte";
import xevents from "../util/xevents";
import { onDestroy, onMount } from "svelte";
import type { TypeCtxMenu } from "../models/ctx_menu";
    import undo from "../util/undo";

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
  const st = !task.is_open;
  backend_adapter.tasks.expand_collapse(st, [task.id]).then(()=>{
    task.is_open = st;
  })
}

const collect_descendants = (parent = task, arr = new Array<TypeTask>()) => {
  for (const child_task of parent.children.values()) {
    console.log(child_task.name, child_task.id);
    arr.push(child_task);
    collect_descendants(child_task, arr);
  }
  return arr;
}
const excol_children = (st: boolean) => {
  const sub_tasks = collect_descendants();
  const ids = sub_tasks.map(t => t.id);
  console.log(sub_tasks);
  backend_adapter.tasks.expand_collapse(st, ids).then(()=>{
    for (const sub_task of sub_tasks) {
      sub_task.is_open = st;
      xevents.emit(`excol:${sub_task.id}`, st);
    } 
  });
}

const listeners = xevents.listen(`excol:${task.id}`, st => {
  console.log('excol event', task.name, task.id, st);
  task.is_open = st; 
}).listen(`ds_update:${task.id}`, (done, started) => {
  task.done = done;
  task.started = started;
});

onDestroy(()=>{
  listeners.cleanup();
})

const open_context_menu = (e: MouseEvent) => {
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
      name: "Toggle Children",
      callback: expand_toggle,
    },
    {
      name: "Expand All Children",
      callback: () => {excol_children(true)}
    },
    {
      name: "Collapse All Children",
      callback: () => {excol_children(false)}
    },
  ];

  const position = {x: e.clientX, y: e.clientY};

  xevents.emit('open_context_menu', {ctx_menu, position})
}

const done_click = () => {
  const original_done = task.done;
  const original_started = task.started;

  const {done, started} = (() => {
    if (task.done) return { done: false, started: false }
    if (task.started) return { done: true, started: true}
    return {done: false, started: true}
  })()

  const affected_tasks = collect_descendants().filter(v=>compare_completion(v, done, started));
  affected_tasks.push(task);
  const ids = affected_tasks.map(t => t.id);

  const do_callback = () => {
    backend_adapter.tasks.update_completion(done, started, ids).then(() => {
      for (const task of affected_tasks) {
        task.done = done;
        task.started = started;
        xevents.emit(`ds_update:${task.id}`, done, started);
      }
    })
  }

  const undo_callback = () => {
    backend_adapter.tasks.update_completion(original_done, original_started, ids).then(() => {
      for (const task of affected_tasks) {
        task.done = done;
        task.started = started;
        xevents.emit(`ds_update:${task.id}`, original_done, original_started);
      }
    })
  }
  undo.do(do_callback, undo_callback);

}

</script>

<main class='main'>
  <main class="body" on:contextmenu|self|preventDefault|stopPropagation={open_context_menu}>
    <Checkbox 
      done={task.done} 
      started={task.started} 
      handle_click={done_click} 
    >
      ({task.id}) {task.name}
    </Checkbox>
    <div class="right">
      <button class='interaction' on:click={invoke_task_creation_wizzard}><span>+</span></button>
      <button class='interaction' on:click={delete_task}><span>-</span></button>
    </div>
  </main>
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

.main {
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
