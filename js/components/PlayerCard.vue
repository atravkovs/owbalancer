<template>
  <div
    class="d-flex justify-content-between w-100"
    draggable="true"
    v-if="player"
    @contextmenu="editPlayer"
    @dragstart="drag"
  >
    <div class="d-flex">
      <div class="w-40p text-center font-smaller">
        <div>
          <rank-icon :rank="sr" />
        </div>
        <div>{{ sr }}</div>
      </div>
      <div class="lh-100">
        {{ player.identity.name }}
        &nbsp;
        <span class="extra-icon">
          <CrownIcon v-if="player.identity.isCaptain" />
          <SwordIcon v-if="player.identity.isSquire" />
        </span>
      </div>
    </div>
    <div class="role-icons">
      <RoleIcon v-for="role in state.icons" :rtype="role" :key="role" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, reactive, computed } from 'vue';

import { useStore } from '@/store';
import PObj, { Player } from '@/objects/player';
import MutationTypes from '@/store/mutation-types';

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
  },
  components: { RoleIcon, RankIcon, CrownIcon, SwordIcon },
  setup(props) {
    const store = useStore();

    const drag = (ev: DragEvent) => {
      let a = null;
      if (ev?.dataTransfer) {
        a = ev.dataTransfer.setData(
          'playerTag',
          props.player?.identity.uuid || ''
        );
        a = ev.dataTransfer?.setData('team', props.teamUuid || '');
      }

      return a;
    };

    const editPlayer = (e: MouseEvent) => {
      e.preventDefault();

      if (props.player === undefined) {
        return;
      }

      store.commit(MutationTypes.EDIT_PLAYER, props.player.identity.uuid);
    };

    const icons = computed(() => {
      const classes = Object.entries(props.player?.stats.classes || {});

      const def = props.prefferedRole ? [props.prefferedRole] : [];

      return classes
        .filter(
          ([rname, role]) =>
            role.isActive &&
            (props.prefferedRole ? rname !== props.prefferedRole : true)
        )
        .sort(([, role], [, role2]) => role2.priority - role.priority)
        .reduce((acc: string[], [name]) => {
          return props.prefferedRole ? [name, ...acc] : [...acc, name];
        }, def);
    });

    const state = reactive({
      icons,
    });

    const sr = computed(() =>
      !props.player ? 0 : props.prefferedRank || PObj.getTopRank(props.player)
    );

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
.lh-100 {
  line-height: 50px;
}
.w-40p {
  width: 40px;
}
.font-smaller {
  font-size: 0.9em;
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
