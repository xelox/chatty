<script lang='ts'>
    import UserIdWrap from "./UserIdWrap.svelte";

export let id: string;
export let tag_type: 'user_mention' | 'channel_mention';
</script>

<span>
  {#if tag_type === 'user_mention'}
    @<UserIdWrap {id}/>
  {/if}
</span>
