export type TeamPlayer = {
    id: string;
    role: 'dps' | 'support' | 'tank';
};

export type Team = {
    name: string;
    averageSR: number;
    players: TeamPlayer[];
};

export type Teams = Team[];

const createEmptyTeam: (name: string) => Team = () => {
    return {
        name,
        averageSR: 0,
        players: [],
    };
};

export default {
    createEmptyTeam
};
