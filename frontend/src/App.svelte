<script lang='ts'>
import Tree from "./lib/Tree.svelte";
import { type NodeType } from "./models/tree";
import backend_adapter from "./util/backend_adapter";

let roots: NodeType[] = [];
backend_adapter.tasks.load().then(res=>{
  roots = res as NodeType[];
})

</script>

<main>
  <div class="tree_wrap">
    {#each roots as root}
      <Tree node={root}/>
    {/each}
  </div>
</main>

<style>
.tree_wrap {
  width: 800px;
  border-right: 1px solid #ddd;
  border-top: 1px solid #ddd;
  margin: 0 auto 0 auto;
}
</style>
