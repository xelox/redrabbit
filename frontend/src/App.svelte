<script lang='ts'>
import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
import TasksToolbar from "./lib/TasksToolbar.svelte";
import backend_adapter from "./util/backend_adapter";
import TaskCollection from "./lib/TaskCollection.svelte";
import xevents from "./util/xevents";
import ContextMenu from "./lib/ContextMenu.svelte";
import xundo from "./util/xundo";

backend_adapter.tasks.load().then(result=>{
  xevents.emit(`add_tasks:root`, result);
})

document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === 'z') {
      xundo.undo();
    }
    
    if (event.ctrlKey && event.key === 'y') {
      xundo.redo();
    }
});

</script>

<main>
  <ContextMenu/>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      <TaskCollection parent={null}/>
    </div>
  </div>
</main>

<TaskCreationWizzard/>

<style>
.vessel {
  width: 800px;
  margin: auto;
}
.tree_wrap {
  width: 100%;
  border-right: 1px solid #ddd;
  border-top: 1px solid #ddd;
  margin: 0 auto 0 auto;
}
</style>
