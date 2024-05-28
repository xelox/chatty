<script lang='ts'>
  abstract class Static {
    public static icons = {
      chat: {
        src: '/svg-files/Communication/comments-alt-2.svg',
        alt: 'Chat Icon'
      },
      plus: {
        src: '/svg-files/Interface and Sign/circle-plus.svg',
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
  const icon = Static.icons[variant];
  
  export let size = "20px";
</script>

<img src={icon.src} alt={icon.alt} title={icon.alt} width={size}> 

<style>
img {
  filter: invert(87%) sepia(6%) saturate(987%) hue-rotate(192deg) brightness(98%) contrast(94%);
  aspect-ratio: 1/1;
  margin: 0;
  pointer-events: none;
}
</style>
