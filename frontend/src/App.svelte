<script lang='ts'>
    import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
    import TasksToolbar from "./lib/TasksToolbar.svelte";
import Tree from "./lib/Tree.svelte";
import { type NodeType } from "./models/tree";
import backend_adapter from "./util/backend_adapter";

let roots: NodeType[] = [];
backend_adapter.tasks.load().then(res=>{
  roots = res as NodeType[];
})

</script>

<main>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      {#each roots as root}
        <Tree node={root}/>
      {/each}
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
