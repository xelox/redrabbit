<script lang='ts'>
    import xevents from "../util/xevents";
import Button from "./Button.svelte";
import Icon from "./Icon.svelte";

const invoke_task_creation_wizzard = () => {
  xevents.emit('invoke_task_creation_wizzard', 'root');
}
</script>

<main>
  <Button click={invoke_task_creation_wizzard} circle={true} color='blue'>
    <Icon variant='plus'/>

  </Button>
</main>

<style>
main {
  margin: 40px auto;
  padding: 16px;
  width: calc(100% - 32px);
  border: 1px solid black;
  border-radius: 2px;
  display: flex;
  justify-content: left;
  align-items: center;
  gap: 10px;
  position: sticky;
  top: 0;
}
</style>
