<script lang='ts'>
  abstract class Static {
    public static icons = {
      check: {
        src: '/svg-files/Interface and Sign/checkmark.svg',
        alt: 'Check Icon'
      },
      minus: {
        src: '/svg-files/Interface and Sign/minus.svg',
        alt: 'Minus Icon'
      },
      close: {
        src: '/svg-files/Interface and Sign/close.svg',
        alt: 'Minus Icon'
      },
      chat: {
        src: '/svg-files/Communication/comments-alt-2.svg',
        alt: 'Chat Icon'
      },
      plus: {
        src: '/svg-files/Interface and Sign/plus.svg',
        alt: 'Plus Icon',
      },
      user: {
        src: '/svg-files/Web and Technology/user.svg',
        alt: 'User Icon',
      },
      settings: {
        src: '/svg-files/Web and Technology/cog.svg',
        alt: 'Settings Icon',
      },
      exit: {
        src: '/svg-files/Direction/exit.svg',
        alt: 'Exit Icon',
      },
      delete: {
        src: '/svg-files/Web and Technology/trash-can.svg',
        alt: 'Delete Icon',
      },
      edit: {
        src: '/svg-files/Education/pencil.svg',
        alt: 'Edit Icon',
      },
      reply: {
        src: '/svg-files/Interface and Sign/more.svg',
        alt: 'Reply Icon',
      },
    } as const
  }

  export let variant: keyof typeof Static.icons;
  $: icon = Static.icons[variant];
  
  export let size = "20px";
  export let color = "white";
</script>

<div style={`mask: url("${icon.src}"); width: ${size}; background-color: ${color};`} title={icon.alt}> </div>

<style>
div {
  mask-size: cover;
  display: inline-block;
  background: red;
  aspect-ratio: 1/1;
  margin: 0;
  pointer-events: none;
}
</style>
