export type TeamMembers = {
    rank: number;
    uuid: string;
    name: string;
    role: 'dps' | 'support' | 'tank';
};

export type Team = {
    name: string;
    avgSr: number;
    totalSr: number;
    members: TeamMembers[];
};

export type Teams = Team[];

const createEmptyTeam: (name: string) => Team = () => {
    return {
        name,
        avgSr: 0,
        totalSr: 0,
        members: [],
    };
};

export default {
    createEmptyTeam
};
