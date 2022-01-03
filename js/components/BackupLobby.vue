<template>
  <div class="w-100 h-100">
    <h3>Backup Lobby</h3>

    <add-player
      :enableRandoms="false"
      :lobby="'backup'"
    />
    <player-list
      class="mt-2"
      :showAll="true"
      :lobby="'backup'"
      :enableExtra="false"
    />
    <delete-player
      class="mt-2"
      :lobby="'backup'"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import AddPlayer from '@/components/Lobby/AddPlayer.vue';
import PlayerList from '@/components/Lobby/PlayerList.vue';
import DeletePlayer from '@/components/Lobby/DeletePlayer.vue';

export default defineComponent({
  name: 'BackupLobby',
  components: { AddPlayer, PlayerList, DeletePlayer },
})
</script>
