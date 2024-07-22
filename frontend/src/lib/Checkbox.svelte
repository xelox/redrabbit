<script lang='ts'>
import Icon from "./Icon.svelte";

export let done: boolean;
export let started: boolean;
let state: 'close' | 'check' | 'minus';

$: {
  if (done) state = 'check';
  else if (started) state = 'minus';
  else state = 'close'
}


abstract class Colors {
  public static readonly i_map = {
    close: '#2b0603',
    minus: '#03122b',
    check: '#0a2b03',
  }
  public static readonly bg_map = {
    close: '#f2d2d2',
    minus: '#d2dbf2',
    check: '#d8f2d2',
  }
}

export let handle_click: (() => void) = () => {
  done = !done;
  if (done) started = true;
}
</script>

<button 
    on:click={handle_click}
>
<div style="background-color: {Colors.bg_map[state]};"> <Icon variant={state} size="10px" color={Colors.i_map[state]}/> </div>
<slot/>
</button>

<style>
button {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  background: transparent;
  font-size: 1em;
}
div {
  width: 1.4em;
  aspect-ratio: 1/1;
  border-radius: 4px;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
}
button:hover {
  color: #4a79e8;
}
</style>
