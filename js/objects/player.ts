import { v4 as uuidv4 } from 'uuid';

export type Identity = {
  uuid: string;
  name: string;
  isSquire: boolean;
  isCaptain: boolean;
};

export type ClassType = {
  priority: number;
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
  rank: number;
  classes: Classes;
};

export type Player = {
  identity: Identity;
  stats: Stats;
  createdAt: Date;
};

export type Players = {
  [tag: string]: Player;
};

const createDefaultPlayer: (name: string) => Player = name => {
  return {
    identity: {
      name,
      uuid: uuidv4(),
      isCaptain: false,
      isSquire: false,
    },
    stats: {
      rank: 0,
      classes: {
        dps: {
          playHours: 0,
          priority: 0,
          primary: false,
          isActive: false,
          secondary: false,
        },
        tank: {
          playHours: 0,
          priority: 1,
          primary: false,
          isActive: false,
          secondary: false,
        },
        support: {
          playHours: 0,
          priority: 2,
          primary: false,
          isActive: false,
          secondary: false,
        },
      },
    },
    createdAt: new Date(),
  };
};

export default {
  createDefaultPlayer,
};
