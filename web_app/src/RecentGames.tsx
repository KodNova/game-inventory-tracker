import { useEffect, useState } from "react";

type Game = {
	id: number;
	name: string;
	released: string | null;
	background_image: string | null;
};

async function getRecentGames(): Promise<Game[]> {
	const res = await fetch(
		`${import.meta.env.VITE_BACKEND_URL}/get_recent_games`,
	);

	if (!res.ok) throw new Error("Failed to fetch recent games");

	const data = await res.json();

	return data.results;
}

export default function RecentGames() {
	const [games, setGames] = useState<Game[]>([]);

	useEffect(() => {
		getRecentGames().then(setGames).catch(console.error);
	}, []);

	return (
		<ul>
			{games.map((game) => (
				<li key={game.id}>{game.name}</li>
			))}
		</ul>
	);
}
