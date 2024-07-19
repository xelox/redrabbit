<script lang='ts'>
export let click: (() => void) | undefined = undefined

abstract class Static {
  static color_map = <const>{
    blue: '#4a79e8',
    red: '#ce4242',
    orange: '#e88f4c',
    green: '#429e3f',
    purple: '#c85ed6',
    yellow: '#d3b943',
  }
}

export let circle = false;
export let color: keyof typeof Static.color_map = 'blue';
</script>

<button on:click={click} class:circle={circle} style="background-color: {Static.color_map[color]}">
  <slot/>
</button>

<style>
button {
  display: flex;
  flex-direction: row;
  padding: 4px 16px;
  border-radius: 2px;
  font-family: 'Noto Sans';
  color: white;
  -webkit-box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24); 
  box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24);
  cursor: pointer;
  transition: 80ms;
}
.circle {
  padding: 4px;
  border-radius: 100%;
  aspect-ratio: 1/1;
  width: max-content;
}
button:hover {
  filter: brightness(1.1);
}
</style>
