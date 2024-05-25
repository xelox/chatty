<script lang="ts">
  export let action: () => void = () => {};
  abstract class Static {
    static bg_colors = <const>{
      blue: '#4a79e8',
      red: '#ce4242',
      orange: '#db7329',
      green: '#429e3f',
      purple: '#d33981',
      yellow: '#d3b943',
    }
  }
  export let bg: keyof typeof Static.bg_colors = 'blue';
</script>

<button on:click={action} style={`background: ${Static.bg_colors[bg]};`}>
  <slot/>
</button>

<style>
button {
  padding: 4px 10px 4px 10px;
  border-radius: 2px;
  font-family: 'Noto Sans';
  color: white;
  -webkit-box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24); 
  box-shadow: 0px 3px 5px -2px rgba(0,0,0,0.24);
  cursor: pointer;
  transition: 80ms;
}
button:hover {
  filter: brightness(1.2);
}
</style>
