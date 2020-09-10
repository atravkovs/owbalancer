<template>
  <div>
    <h3>Lobby</h3>

    <AddPlayer />
    <PlayerList class="mt-2" />
    <DeletePlayer class="mt-2" />
    <edit-player />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import AddPlayer from '@/components/AddPlayer.vue';
import PlayerList from '@/components/PlayerList.vue';
import DeletePlayer from '@/components/DeletePlayer.vue';
import EditPlayer from '@/components/EditPlayer.vue';

export default defineComponent({
  name: 'Lobby',
  components: { AddPlayer, PlayerList, DeletePlayer, EditPlayer },
});
</script>
