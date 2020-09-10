export type Identity = {
  battleTag: string;
  displayName: string;
  twitchName: string;
  isSquire: boolean;
  isCaptain: boolean;
};

export type ClassType = {
  rating: number;
  priority: number;
  playHours: number;
  isActive: boolean;
  primary: boolean;
  secondary: boolean;
};

export type DescribedClassType = ClassType & { role: string };

export type Classes = {
  dps: ClassType;
  tank: ClassType;
  support: ClassType;
};

export type Stats = {
  level: number;
  classes: Classes;
};

export type Player = {
  identity: Identity;
  stats: Stats;
};

export type Players = {
  [tag: string]: Player;
};

const createDefaultPlayer: (battleTag: string) => Player = battleTag => {
  const displayName = battleTag.split('#')[0];

  return {
    identity: {
      battleTag,
      displayName,
      twitchName: '',
      isCaptain: false,
      isSquire: false,
    },
    stats: {
      level: 0,
      classes: {
        dps: {
          rating: 0,
          playHours: 0,
          isActive: false,
          priority: 0,
          primary: false,
          secondary: false,
        },
        tank: {
          rating: 0,
          playHours: 0,
          isActive: false,
          priority: 1,
          primary: false,
          secondary: false,
        },
        support: {
          rating: 0,
          playHours: 0,
          isActive: false,
          priority: 2,
          primary: false,
          secondary: false,
        },
      },
    },
  };
};

export default {
  createDefaultPlayer,
};
