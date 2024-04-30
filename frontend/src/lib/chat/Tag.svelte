<script lang='ts'>
export let id: string;
export let tag_type: 'user_mention' | 'channel_mention';
</script>

<main>
  ({tag_type} {id})
</main>
