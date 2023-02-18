<template>
  <div
    class="d-flex justify-content-between w-100"
    draggable="true"
    v-if="player"
    @contextmenu="editPlayer"
    @dragstart="drag"
  >
    <div class="d-flex">
      <div class="text-center lh-80" :class="{ 'd-flex': !!teamUuid, 'w-40p': !teamUuid }">
        <div>
          <rank-icon :rank="sr" />
        </div>
      </div>
      <div class="text-ellip" :class="{ 'lh-100': !teamUuid, 'ps-1': !!teamUuid, wt: !!teamUuid }">
        <span class="extra-icon">
          <crown-icon v-if="player.identity.isCaptain" />
          <sword-icon v-if="player.identity.isSquire" />
        </span>
        {{ player.identity.name }}
      </div>
    </div>
    <div class="role-icons">
      <lock-icon v-if="player.identity.isLocked" />
      <role-icon v-for="role in state.icons" :rtype="role" :key="role" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, reactive, computed } from 'vue';

import { useStore } from '@/store';
import PObj, { LobbyType, Player } from '@/objects/player';
import MutationTypes from '@/store/mutation-types';

import LockIcon from '@/components/svg/LockIcon.vue';
import RoleIcon from '@/components/svg/RoleIcon.vue';
import RankIcon from '@/components/svg/RankIcon.vue';
import CrownIcon from '@/components/svg/CrownIcon.vue';
import SwordIcon from '@/components/svg/SwordIcon.vue';

export default defineComponent({
  name: 'PlayerCard',
  props: {
    teamUuid: String,
    player: Object as PropType<Player>,
    prefferedRole: String,
    prefferedRank: Number,
    rankRole: String,
    lobby: {
      type: String as PropType<LobbyType>,
      default: 'players'
    }
  },
  components: { RoleIcon, RankIcon, CrownIcon, SwordIcon, LockIcon },
  setup(props) {
    const store = useStore();

    const drag = (ev: DragEvent) => {
      let a = null;
      if (ev?.dataTransfer) {
        a = ev.dataTransfer.setData('playerTag', props.player?.identity.uuid || '');
        a = ev.dataTransfer.setData('team', props.teamUuid || '');
        a = ev.dataTransfer.setData('from', props.lobby || '');
      }

      return a;
    };

    const editPlayer = (e: MouseEvent) => {
      e.preventDefault();

      if (props.player === undefined) {
        return;
      }

      store.commit(MutationTypes.EDIT_PLAYER, { playerId: props.player.identity.uuid, lobby: props.lobby });
    };

    const icons = computed(() => {
      const classes = Object.entries(props.player?.stats.classes || {});

      const def = props.prefferedRole ? [props.prefferedRole] : [];

      return classes
        .filter(
          ([rname, role]) =>
            role.isActive && (props.prefferedRole ? rname !== props.prefferedRole : true)
        )
        .sort(([, role], [, role2]) => role2.priority - role.priority)
        .reduce((acc: string[], [name]) => {
          return props.prefferedRole ? [name, ...acc] : [...acc, name];
        }, def);
    });

    const state = reactive({
      icons,
    });

    const sr = computed(() => {
      if (!props.player) return 0;

      if (props.rankRole) return PObj.getRole(props.player.stats.classes, props.rankRole).rank;

      if (props.prefferedRank) return props.prefferedRank;

      return PObj.getTopRank(props.player);
    });

    return {
      sr,
      drag,
      state,
      editPlayer,
    };
  },
});
</script>

<style lang="scss" scoped>
.text-ellip {
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}
.wt {
  width: 175px;
}
.lh-100 {
  line-height: 50px;
}
.lh-80 {
  line-height: 41px;
}
.w-40p {
  width: 40px;
}
.role-icons {
  line-height: 1;
}
.role-icons > * {
  font-size: 0.8em;
  color: var(--bs-gray);
}
.role-icons > *:last-child {
  font-size: 1em;
  color: var(--bs-gray-dark);
}
.extra-icon {
  color: var(--bs-info);
}
</style>
