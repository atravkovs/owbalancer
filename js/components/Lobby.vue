<template>
  <div class="w-100 h-100">
    <h3>Lobby</h3>

    <add-player />
    <player-list class="mt-2" />
    <delete-player class="mt-2" />
    <edit-player />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import AddPlayer from '@/components/Lobby/AddPlayer.vue';
import PlayerList from '@/components/Lobby/PlayerList.vue';
import DeletePlayer from '@/components/Lobby/DeletePlayer.vue';
import EditPlayer from '@/components/Lobby/EditPlayer.vue';

export default defineComponent({
  name: 'Lobby',
  components: { AddPlayer, PlayerList, DeletePlayer, EditPlayer },
});
</script>
